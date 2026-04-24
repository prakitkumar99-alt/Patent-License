#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short,
    token, Address, Env, String, Symbol, Vec,
};

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Patent {
    pub patent_id: u64,
    pub owner: Address,
    pub title: String,
    pub license_fee: i128,
    pub royalty_rate_bps: u32,
    pub is_active: bool,
    pub total_collected: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct License {
    pub license_id: u64,
    pub patent_id: u64,
    pub licensee: Address,
    pub expiry_ledger: u32,
    pub is_valid: bool,
}

#[contracttype]
pub enum DataKey {
    Patent(u64),
    License(u64),
    LicenseeMap(Address),
    PatentLicenses(u64),
    Token, // The token used for payments (e.g., XLM)
    Admin,
}

const PATENT_COUNT: Symbol = symbol_short!("PAT_CNT");
const LICENSE_COUNT: Symbol = symbol_short!("LIC_CNT");

#[contract]
pub struct PatentLicenseContract;

#[contractimpl]
impl PatentLicenseContract {
    pub fn initialize(env: Env, admin: Address, token_address: Address) {
        if env.storage().instance().has(&DataKey::Admin) {
            panic!("already initialized");
        }
        env.storage().instance().set(&DataKey::Admin, &admin);
        env.storage().instance().set(&DataKey::Token, &token_address);
        env.storage().instance().set(&PATENT_COUNT, &0u64);
        env.storage().instance().set(&LICENSE_COUNT, &0u64);
    }

    pub fn register_patent(
        env: Env,
        owner: Address,
        title: String,
        license_fee: i128,
        royalty_rate_bps: u32,
    ) -> u64 {
        owner.require_auth();

        if license_fee <= 0 || royalty_rate_bps > 10_000 {
            panic!("invalid terms");
        }

        let mut count: u64 = env.storage().instance().get(&PATENT_COUNT).unwrap_or(0);
        count += 1;

        let patent = Patent {
            patent_id: count,
            owner: owner.clone(),
            title,
            license_fee,
            royalty_rate_bps,
            is_active: true,
            total_collected: 0,
        };

        let key = DataKey::Patent(count);
        env.storage().persistent().set(&key, &patent);
        // Extend TTL so the patent doesn't expire from storage
        env.storage().persistent().extend_ttl(&key, 5000, 10000);
        
        env.storage().instance().set(&PATENT_COUNT, &count);
        count
    }

    pub fn purchase_license(
        env: Env,
        licensee: Address,
        patent_id: u64,
        duration_ledgers: u32,
    ) -> u64 {
        licensee.require_auth();

        let patent_key = DataKey::Patent(patent_id);
        let mut patent: Patent = env.storage().persistent().get(&patent_key).expect("not found");

        if !patent.is_active {
            panic!("patent inactive");
        }

        // ── ACTUAL TOKEN TRANSFER ──
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&licensee, &patent.owner, &patent.license_fee);

        let mut lic_count: u64 = env.storage().instance().get(&LICENSE_COUNT).unwrap_or(0);
        lic_count += 1;

        let expiry_ledger = env.ledger().sequence().checked_add(duration_ledgers).expect("overflow");

        let license = License {
            license_id: lic_count,
            patent_id,
            licensee: licensee.clone(),
            expiry_ledger,
            is_valid: true,
        };

        // Update Patent stats
        patent.total_collected += patent.license_fee;
        env.storage().persistent().set(&patent_key, &patent);

        // Save License
        let lic_key = DataKey::License(lic_count);
        env.storage().persistent().set(&lic_key, &license);
        env.storage().persistent().extend_ttl(&lic_key, 5000, 10000);

        env.storage().instance().set(&LICENSE_COUNT, &lic_count);

        // Update Mappings
        let mut l_map: Vec<u64> = env.storage().persistent().get(&DataKey::LicenseeMap(licensee.clone())).unwrap_or(Vec::new(&env));
        l_map.push_back(lic_count);
        env.storage().persistent().set(&DataKey::LicenseeMap(licensee), &l_map);

        lic_count
    }

    pub fn record_royalty(
        env: Env,
        licensee: Address,
        patent_id: u64,
        usage_amount: i128,
    ) -> i128 {
        licensee.require_auth();

        let patent_key = DataKey::Patent(patent_id);
        let mut patent: Patent = env.storage().persistent().get(&patent_key).expect("not found");

        let royalty_due = (usage_amount * patent.royalty_rate_bps as i128) / 10_000;

        // ── ACTUAL TOKEN TRANSFER FOR ROYALTIES ──
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let client = token::Client::new(&env, &token_addr);
        client.transfer(&licensee, &patent.owner, &royalty_due);

        patent.total_collected += royalty_due;
        env.storage().persistent().set(&patent_key, &patent);

        royalty_due
    }

    pub fn is_license_valid(env: Env, license_id: u64) -> bool {
        let license: License = env.storage().persistent().get(&DataKey::License(license_id)).expect("not found");
        license.is_valid && env.ledger().sequence() <= license.expiry_ledger
    }

    // ... (Keep other getter functions as they were)
}
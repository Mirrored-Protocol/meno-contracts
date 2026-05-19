#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, Symbol, Address, Map};

#[contracttype]
#[derive(Clone)]
pub struct Listing {
    pub seller: Address,
    pub price: i128,
    pub active: bool,
}

#[contracttype]
pub enum DataKey {
    Listing(Symbol),
    ListingCount,
}

#[contract]
pub struct Marketplace;

#[contractimpl]
impl Marketplace {
    pub fn list_asset(env: Env, seller: Address, asset_id: Symbol, price: i128) {
        seller.require_auth();

        let listing = Listing {
            seller: seller.clone(),
            price,
            active: true,
        };

        env.storage().persistent().set(&DataKey::Listing(asset_id.clone()), &listing);

        let count: u64 = env.storage().persistent().get(&DataKey::ListingCount).unwrap_or(0);
        env.storage().persistent().set(&DataKey::ListingCount, &(count + 1));

        env.events().publish((symbol_short!("listed"), seller), (asset_id, price));
    }

    pub fn buy_asset(env: Env, buyer: Address, asset_id: Symbol) {
        buyer.require_auth();

        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&DataKey::Listing(asset_id.clone()))
            .expect("Listing not found");

        assert!(listing.active, "Listing is no longer active");

        listing.active = false;
        env.storage().persistent().set(&DataKey::Listing(asset_id.clone()), &listing);

        env.events().publish((symbol_short!("bought"), buyer), asset_id);
    }

    pub fn cancel_listing(env: Env, seller: Address, asset_id: Symbol) {
        seller.require_auth();

        let mut listing: Listing = env
            .storage()
            .persistent()
            .get(&DataKey::Listing(asset_id.clone()))
            .expect("Listing not found");

        assert!(listing.seller == seller, "Not the listing owner");
        listing.active = false;
        env.storage().persistent().set(&DataKey::Listing(asset_id.clone()), &listing);
    }

    pub fn get_listing(env: Env, asset_id: Symbol) -> Option<Listing> {
        env.storage().persistent().get(&DataKey::Listing(asset_id))
    }

    pub fn listing_count(env: Env) -> u64 {
        env.storage().persistent().get(&DataKey::ListingCount).unwrap_or(0)
    }
}

mod test;

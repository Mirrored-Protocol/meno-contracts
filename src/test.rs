#![cfg(test)]
use super::*;
use soroban_sdk::{testutils::Address as _, Env, symbol_short};

#[test]
fn test_list_asset_stores_listing() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);

    let seller = Address::generate(&env);
    let asset_id = symbol_short!("asset1");
    let price = 100_i128;

    client.list_asset(&seller, &asset_id, &price);

    let listing = client.get_listing(&asset_id).expect("Listing should exist");
    assert!(listing.active);
    assert_eq!(listing.price, price);
    assert_eq!(listing.seller, seller);
    assert_eq!(client.listing_count(), 1);
}

#[test]
fn test_buy_asset_deactivates_listing() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);

    let seller = Address::generate(&env);
    let buyer = Address::generate(&env);
    let asset_id = symbol_short!("asset1");

    client.list_asset(&seller, &asset_id, &100_i128);
    client.buy_asset(&buyer, &asset_id);

    let listing = client.get_listing(&asset_id).expect("Listing should still exist");
    assert!(!listing.active);
}

#[test]
fn test_cancel_listing() {
    let env = Env::default();
    env.mock_all_auths();
    let contract_id = env.register_contract(None, Marketplace);
    let client = MarketplaceClient::new(&env, &contract_id);

    let seller = Address::generate(&env);
    let asset_id = symbol_short!("asset1");

    client.list_asset(&seller, &asset_id, &50_i128);
    client.cancel_listing(&seller, &asset_id);

    let listing = client.get_listing(&asset_id).expect("Listing should exist");
    assert!(!listing.active);
}

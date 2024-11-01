#![cfg(test)]

use super::*;
use soroban_sdk::{
    testutils::{Address as _, MockAuth, MockAuthInvoke},
    token::Client as TokenClient,
    Address, Env, IntoVal,
};

#[test]
fn test_hold_funds() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let usdc_amount = 1000i128;

    // Register the USDC token contract
    let usdc_contract_address = env.register_stellar_asset_contract(buyer.clone());
    let usdc_token_client = TokenClient::new(&env, &usdc_contract_address);

    // Mint USDC to the buyer
    usdc_token_client.mint(&buyer, &usdc_amount);

    // Ensure the buyer has the correct balance
    assert_eq!(usdc_token_client.balance(&buyer), usdc_amount);

    // Hold funds in escrow
    escrow::hold_funds(&env, &buyer, usdc_amount).unwrap();

    // Check that the funds have been transferred to the escrow contract
    assert_eq!(usdc_token_client.balance(&buyer), 0);
    assert_eq!(usdc_token_client.balance(&env.current_contract_address()), usdc_amount);
}

#[test]
fn test_release_funds() {
    let env = Env::default();
    env.mock_all_auths();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);
    let usdc_amount = 1000i128;

    // Register the USDC token contract
    let usdc_contract_address = env.register_stellar_asset_contract(buyer.clone());
    let usdc_token_client = TokenClient::new(&env, &usdc_contract_address);

    // Mint USDC to the buyer and hold in escrow
    usdc_token_client.mint(&buyer, &usdc_amount);
    escrow::hold_funds(&env, &buyer, usdc_amount).unwrap();

    // Release funds to the seller
    escrow::release_funds(&env, &seller, usdc_amount).unwrap();

    // Check that the funds have been transferred to the seller
    assert_eq!(usdc_token_client.balance(&env.current_contract_address()), 0);
    assert_eq!(usdc_token_client.balance(&seller), usdc_amount);
}

#[test]
#[should_panic(expected = "AuthorizationFailed")]
fn test_hold_funds_unauthorized() {
    let env = Env::default();

    let buyer = Address::generate(&env);
    let usdc_amount = 1000i128;

    // Attempt to hold funds without authorization
    escrow::hold_funds(&env, &buyer, usdc_amount).unwrap();
}

#[test]
#[should_panic(expected = "AuthorizationFailed")]
fn test_release_funds_unauthorized() {
    let env = Env::default();

    let seller = Address::generate(&env);
    let usdc_amount = 1000i128;

    // Attempt to release funds without authorization
    escrow::release_funds(&env, &seller, usdc_amount).unwrap();
}

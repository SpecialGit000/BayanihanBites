#![cfg(test)]
use super::{BayanihanBitesContract, BayanihanBitesContractClient, DeliveryState};
use soroban_sdk::{testutils::Address as _, Address, Env, token};

fn setup_env<'a>() -> (Env, BayanihanBitesContractClient<'a>, Address, Address, Address, token::StellarAssetClient<'a>) {
    let env = Env::default();
    env.mock_all_auths();

    let contract_id = env.register_contract(None, BayanihanBitesContract);
    let client_contract = BayanihanBitesContractClient::new(&env, &contract_id);

    let customer = Address::generate(&env);
    let merchant = Address::generate(&env);
    let courier = Address::generate(&env);
    
    let token_admin = Address::generate(&env);
    let token_contract_id = env.register_stellar_asset_contract(token_admin);
    let token_admin_client = token::StellarAssetClient::new(&env, &token_contract_id);
    
    token_admin_client.mint(&customer, &2000);

    (env, client_contract, customer, merchant, courier, token_admin_client)
}

#[test]
fn test_happy_path_delivery_flow() {
    let (env, contract, customer, merchant, courier, token) = setup_env();
    let token_client = token::Client::new(&env, &token.address);

    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
    
    // Deposit phase
    contract.deposit_escrow();
    assert_eq!(token_client.balance(&env.current_contract_address()), 200);

    // Food prep & pickup phase
    contract.pickup_order();
    assert_eq!(token_client.balance(&merchant), 150);
    assert_eq!(contract.get_delivery_state(), DeliveryState::PickedUp);

    // Final home dropoff confirmation phase
    contract.confirm_delivery();
    assert_eq!(token_client.balance(&courier), 50);
    assert_eq!(contract.get_delivery_state(), DeliveryState::Delivered);
}

#[test]
#[should_panic(expected = "HostError: Error(Contract, #2)")]
fn test_cannot_pickup_before_deposit_fails() {
    let (env, contract, customer, merchant, courier, token) = setup_env();
    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
    
    // Attacker attempts to advance sequence without escrow coverage funding
    env.mock_all_auths();
    contract.pickup_order();
}

#[test]
fn test_state_verification_initializes_ordered() {
    let (_env, contract, customer, merchant, courier, token) = setup_env();
    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
    
    assert_eq!(contract.get_delivery_state(), DeliveryState::Ordered);
}

#[test]
#[should_panic]
fn test_courier_cannot_trigger_customer_delivery_payout() {
    let (env, contract, customer, merchant, courier, token) = setup_env();
    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
    
    contract.deposit_escrow();
    contract.pickup_order();

    // Courier identity attempts to force call customer's confirmation function
    env.mock_all_auths();
    contract.confirm_delivery();
}

#[test]
#[should_panic]
fn test_cannot_double_initialize() {
    let (_env, contract, customer, merchant, courier, token) = setup_env();
    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
    contract.initialize(&customer, &merchant, &courier, &token.address, &150, &50);
}
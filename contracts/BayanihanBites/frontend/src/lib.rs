#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, token, Address, Env};

#[contracttype]
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DeliveryState {
    Ordered = 0,
    PickedUp = 1,
    Delivered = 2,
    Cancelled = 3,
}

#[contracttype]
#[derive(Clone)]
pub enum DataKey {
    Customer,
    Merchant,
    Courier,
    Token,
    FoodCost,
    DeliveryFee,
    State,
}

#[contract]
pub struct BayanihanBitesContract;

#[contractimpl]
impl BayanihanBitesContract {
    /// Initializes a locked logistics milestone delivery order.
    pub fn initialize(
        env: Env,
        customer: Address,
        merchant: Address,
        courier: Address,
        token: Address,
        food_cost: i128,
        delivery_fee: i128,
    ) {
        assert!(!env.storage().instance().has(&DataKey::Customer), "Order already initialized");
        assert!(food_cost > 0 && delivery_fee > 0, "Financial inputs must be positive");

        env.storage().instance().set(&DataKey::Customer, &customer);
        env.storage().instance().set(&DataKey::Merchant, &merchant);
        env.storage().instance().set(&DataKey::Courier, &courier);
        env.storage().instance().set(&DataKey::Token, &token);
        env.storage().instance().set(&DataKey::FoodCost, &food_cost);
        env.storage().instance().set(&DataKey::DeliveryFee, &delivery_fee);
        env.storage().instance().set(&DataKey::State, &DeliveryState::Ordered);
    }

    /// Customer confirms order and deposits combined cost into escrow.
    pub fn deposit_escrow(env: Env) {
        let customer: Address = env.storage().instance().get(&DataKey::Customer).unwrap();
        customer.require_auth();

        let state: DeliveryState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == DeliveryState::Ordered, "Invalid escrow sequence state");

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let food_cost: i128 = env.storage().instance().get(&DataKey::FoodCost).unwrap();
        let delivery_fee: i128 = env.storage().instance().get(&DataKey::DeliveryFee).unwrap();
        
        let total_deposit = food_cost + delivery_fee;
        let payment_token = token::Client::new(&env, &token_addr);
        
        payment_token.transfer(&customer, &env.current_contract_address(), &total_deposit);
    }

    /// Step 1: Courier arrives at merchant, picks up hot food. Merchant gets paid instantly.
    pub fn pickup_order(env: Env) {
        let merchant: Address = env.storage().instance().get(&DataKey::Merchant).unwrap();
        merchant.require_auth();

        let state: DeliveryState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == DeliveryState::Ordered, "Order cannot be picked up");

        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let food_cost: i128 = env.storage().instance().get(&DataKey::FoodCost).unwrap();
        let payment_token = token::Client::new(&env, &token_addr);

        // Instant settlement to merchant for goods protection
        payment_token.transfer(&env.current_contract_address(), &merchant, &food_cost);
        env.storage().instance().set(&DataKey::State, &DeliveryState::PickedUp);
    }

    /// Step 2: Courier delivers food to doorstep. Courier gets paid delivery fee instantly.
    pub fn confirm_delivery(env: Env) {
        let customer: Address = env.storage().instance().get(&DataKey::Customer).unwrap();
        customer.require_auth();

        let state: DeliveryState = env.storage().instance().get(&DataKey::State).unwrap();
        assert!(state == DeliveryState::PickedUp, "Delivery not in transit state");

        let courier: Address = env.storage().instance().get(&DataKey::Courier).unwrap();
        let token_addr: Address = env.storage().instance().get(&DataKey::Token).unwrap();
        let delivery_fee: i128 = env.storage().instance().get(&DataKey::DeliveryFee).unwrap();
        let payment_token = token::Client::new(&env, &token_addr);

        // Courier receives tips and base logistics base pay split
        payment_token.transfer(&env.current_contract_address(), &courier, &delivery_fee);
        env.storage().instance().set(&DataKey::State, &DeliveryState::Delivered);
    }

    /// Safe check utility for tracking order tracking flow status.
    pub fn get_delivery_state(env: Env) -> DeliveryState {
        env.storage().instance().get(&DataKey::State).unwrap_or(DeliveryState::Ordered)
    }
}
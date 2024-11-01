#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Env};

#[test]
fn test_set_and_get_transaction_state() {
    let env = Env::default();

    // Set the transaction state to Initiated
    state::set_transaction_state(&env, state::TransactionState::Initiated).unwrap();
    assert_eq!(
        state::get_transaction_state(&env).unwrap(),
        state::TransactionState::Initiated
    );

    // Change the transaction state to FundsHeld
    state::set_transaction_state(&env, state::TransactionState::FundsHeld).unwrap();
    assert_eq!(
        state::get_transaction_state(&env).unwrap(),
        state::TransactionState::FundsHeld
    );

    // Change the transaction state to Completed
    state::set_transaction_state(&env, state::TransactionState::Completed).unwrap();
    assert_eq!(
        state::get_transaction_state(&env).unwrap(),
        state::TransactionState::Completed
    );
}

#[test]
#[should_panic(expected = "StateNotSet")]
fn test_get_transaction_state_not_set() {
    let env = Env::default();

    // Attempt to get a transaction state that hasn't been set
    state::get_transaction_state(&env).unwrap();
}

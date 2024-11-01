use soroban_sdk::{Env, Symbol};
use crate::errors::EscrowError;

/// Enum representing the different states of a transaction.
#[derive(Debug, PartialEq, Eq)]
pub enum TransactionState {
    Initiated,
    FundsHeld,
    PaymentPending,
    Completed,
    Disputed,
}

/// Sets the transaction state in the contract's storage.
/// This function updates the current state of the transaction.
pub fn set_transaction_state(env: &Env, state: TransactionState) -> Result<(), EscrowError> {
    let state_key = get_state_key();
    env.storage().set(&state_key, &state);

    Ok(())
}

/// Retrieves the current transaction state from the contract's storage.
/// This function returns the current state or an error if not set.
pub fn get_transaction_state(env: &Env) -> Result<TransactionState, EscrowError> {
    let state_key = get_state_key();
    env.storage().get(&state_key).ok_or(EscrowError::StateNotSet)
}

/// Generates a unique storage key for the transaction state.
/// This function creates a key for storing the transaction state.
fn get_state_key() -> Symbol {
    Symbol::short("transaction_state")
}

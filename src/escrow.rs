use soroban_sdk::{contractimpl, Address, Env, Symbol};
use crate::state::{self, TransactionState};
use crate::errors::EscrowError;
use crate::utils;

/// Holds USDC funds in escrow for a transaction.
/// This function is called when a transaction is initiated.
pub fn hold_funds(env: &Env, buyer: &Address, usdc_amount: i128) -> Result<(), EscrowError> {
    // Ensure the buyer has authorized this action
    buyer.require_auth();

    // Transfer USDC from the buyer to the escrow contract
    let usdc_token = utils::get_usdc_token(env);
    let contract_address = env.current_contract_address();
    usdc_token.transfer(buyer, &contract_address, &usdc_amount)?;

    // Update the transaction state to indicate funds are held
    state::set_transaction_state(env, TransactionState::FundsHeld)?;

    Ok(())
}

/// Releases USDC funds from escrow to the seller.
/// This function is called when both parties confirm the transaction.
pub fn release_funds(env: &Env, seller: &Address, usdc_amount: i128) -> Result<(), EscrowError> {
    // Ensure the seller has authorized this action
    seller.require_auth();

    // Transfer USDC from the escrow contract to the seller
    let usdc_token = utils::get_usdc_token(env);
    let contract_address = env.current_contract_address();
    usdc_token.transfer(&contract_address, seller, &usdc_amount)?;

    // Update the transaction state to indicate funds are released
    state::set_transaction_state(env, TransactionState::Completed)?;

    Ok(())
}

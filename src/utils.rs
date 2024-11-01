use soroban_sdk::{Env, Address, Symbol, contracterror};
use crate::errors::EscrowError;

/// Validates the input parameters for a transaction.
/// This function checks that the buyer and seller addresses are valid and that the USDC amount is positive.
pub fn validate_input(env: &Env, buyer: &Address, seller: &Address, usdc_amount: i128) -> Result<(), EscrowError> {
    if buyer == seller {
        return Err(EscrowError::InvalidInput);
    }
    if usdc_amount <= 0 {
        return Err(EscrowError::InvalidInput);
    }
    Ok(())
}

/// Retrieves the USDC token client for performing token operations.
/// This function assumes that the USDC token contract is deployed and accessible.
pub fn get_usdc_token(env: &Env) -> soroban_sdk::token::Client {
    // Assuming the USDC token contract address is known and set in the environment
    let usdc_contract_address = env.get_contract_data::<Address>(&Symbol::short("usdc_contract")).unwrap();
    soroban_sdk::token::Client::new(env, &usdc_contract_address)
}

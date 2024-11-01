use soroban_sdk::{Env, Address};
use crate::errors::EscrowError;

/// Enum representing the roles in the marketplace.
#[derive(Debug, PartialEq, Eq)]
pub enum Role {
    Buyer,
    Seller,
}

/// Assigns a role to a participant in the transaction.
/// This function ensures that the correct role is assigned based on the participant's address.
pub fn assign_role(env: &Env, participant: &Address, role: Role) -> Result<(), EscrowError> {
    // Store the role assignment in the contract's storage
    let role_key = get_role_key(participant);
    env.storage().set(&role_key, &role);

    Ok(())
}

/// Retrieves the role of a participant.
/// This function checks the contract's storage to determine the role of the given address.
pub fn get_role(env: &Env, participant: &Address) -> Result<Role, EscrowError> {
    let role_key = get_role_key(participant);
    env.storage().get(&role_key).ok_or(EscrowError::RoleNotAssigned)
}

/// Generates a unique storage key for a participant's role.
/// This function creates a key based on the participant's address to store their role.
fn get_role_key(participant: &Address) -> Vec<u8> {
    let mut key = Vec::new();
    key.extend_from_slice(b"role:");
    key.extend_from_slice(participant.as_bytes());
    key
}

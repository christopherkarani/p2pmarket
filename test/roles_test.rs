#![cfg(test)]

use super::*;
use soroban_sdk::{testutils::Address as _, Address, Env};

#[test]
fn test_assign_and_get_role() {
    let env = Env::default();

    let buyer = Address::generate(&env);
    let seller = Address::generate(&env);

    // Assign roles to buyer and seller
    roles::assign_role(&env, &buyer, roles::Role::Buyer).unwrap();
    roles::assign_role(&env, &seller, roles::Role::Seller).unwrap();

    // Retrieve and verify roles
    assert_eq!(roles::get_role(&env, &buyer).unwrap(), roles::Role::Buyer);
    assert_eq!(roles::get_role(&env, &seller).unwrap(), roles::Role::Seller);
}

#[test]
#[should_panic(expected = "RoleNotAssigned")]
fn test_get_role_not_assigned() {
    let env = Env::default();

    let participant = Address::generate(&env);

    // Attempt to get a role for a participant who hasn't been assigned one
    roles::get_role(&env, &participant).unwrap();
}

#[test]
fn test_reassign_role() {
    let env = Env::default();

    let participant = Address::generate(&env);

    // Assign a role and then reassign a different role
    roles::assign_role(&env, &participant, roles::Role::Buyer).unwrap();
    assert_eq!(roles::get_role(&env, &participant).unwrap(), roles::Role::Buyer);

    roles::assign_role(&env, &participant, roles::Role::Seller).unwrap();
    assert_eq!(roles::get_role(&env, &participant).unwrap(), roles::Role::Seller);
}

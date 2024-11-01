# P2P Marketplace Escrow Smart Contract

## Project Overview

The P2P Marketplace Escrow smart contract facilitates secure and efficient transactions between buyers and sellers trading USDC stablecoins for Kenyan Shillings. The contract acts as an escrow, holding USDC until both parties confirm the transaction.

### Key Features
- **Secure Escrow**: Holds USDC in escrow until transaction confirmation.
- **Role Management**: Differentiates between buyers and sellers.
- **Payment Confirmation**: Ensures payment is confirmed before releasing funds.
- **Time Constraints**: Enforces deadlines for payment confirmation.
- **Dispute Resolution**: Mechanism for handling transaction disputes.

### Prerequisites
- Rust and Cargo installed
- Soroban SDK

## Setup & Installation

### Dependencies
Ensure you have the following dependencies installed:
- Rust
- Cargo
- Soroban SDK

### Build Instructions
1. Clone the repository:
   2. Build the project:
   
### Deployment Steps
1. Deploy the contract using Soroban CLI:
   
## Usage Guide

### Main Functions
- **initialize**: Sets up the contract.
- **execute_transaction**: Manages the escrow process for a transaction.

### Common Operations
- **Hold Funds**: Call `execute_transaction` to initiate a transaction and hold funds in escrow.
- **Release Funds**: Funds are automatically released upon confirmation by both parties.

### Important Parameters
- `buyer`: Address of the buyer.
- `seller`: Address of the seller.
- `usdc_amount`: Amount of USDC to be held in escrow.
- `payment_deadline`: Deadline for payment confirmation.

## Contract Structure

### Key Files
- **src/lib.rs**: Main entry point for the contract.
- **src/escrow.rs**: Manages the escrow process.
- **src/roles.rs**: Handles role management.
- **src/state.rs**: Manages transaction states.
- **src/errors.rs**: Defines error handling.
- **src/utils.rs**: Provides utility functions.

### Main Components
- **Escrow Management**: Handles holding and releasing of funds.
- **Role Management**: Assigns and verifies roles of participants.
- **State Management**: Tracks the state of transactions.

## Testing

### How to Run Tests
Run the tests using Cargo:

### Test Coverage Overview
- **escrow_test.rs**: Tests escrow functionality including fund holding and releasing.
- **roles_test.rs**: Tests role assignment and retrieval.
- **state_test.rs**: Tests state transitions and retrieval.

## Security Considerations
- Ensure proper authorization for all transactions.
- Validate all inputs to prevent invalid transactions.
- Implement timeouts and dispute resolution to handle edge cases.

## Troubleshooting
- Ensure all dependencies are installed and up to date.
- Verify that the Soroban SDK is correctly configured.
- Check error messages for specific issues during deployment or execution.

This documentation provides a comprehensive guide to understanding, setting up, and using the P2P Marketplace Escrow smart contract. For further assistance, please refer to the Soroban SDK documentation or contact the project maintainers.
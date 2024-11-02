# P2P Marketplace Escrow Smart Contract

## Project Overview

The P2P Marketplace Escrow smart contract facilitates secure and efficient transactions between buyers and sellers trading USDC stablecoins for Fiat. The contract acts as an escrow, holding USDC until both parties confirm the transaction.

### User Story

## User Story: Buyer on Bebop App P2P Marketplace

#### As a Buyer who wants to purchase cryptocurrency through Bebop App’s P2P Marketplace, I want a simple, secure, and reliable way to purchase digital assets directly from other users so that I can acquire crypto quickly using my preferred payment method.

## Discovery and Selection:
   - I log into my Bebop App account and navigate to the P2P platform.
   - I filter by the cryptocurrency I want to buy, as well as my preferred currency and payment method (e.g., bank transfer, PayPal).
   - I view a list of available sellers along with their rates, completion percentages, limits, and user ratings.
   - I select a seller who has good ratings, offers my preferred payment method, and has a competitive rate.

## Initiating the Trade:

   - I enter the amount of cryptocurrency I wish to purchase and click “Buy.”
   - The trade is locked, and the cryptocurrency amount is temporarily held in escrow by a smart contract, ensuring my payment is secure.

## Making Payment:
   - I follow the instructions provided by the seller to make the payment through the agreed payment method.
   - After sending the payment, I return to the Bebop App and click “Mark as Paid,” signaling the seller to verify the payment

## Confirming Receipt:
   - Once the seller confirms the payment, the smart contract releases the cryptocurrency from escrow and into my wallet.
   - I receive a notification, check my wallet to ensure the funds are there, and rate the seller based on my experience.
## Outcome:
   - I have successfully purchased cryptocurrency through a quick and secure process.







## User Story: Seller on Bebop App P2P Marketplace

#### As a Seller who wants to exchange cryptocurrency for fiat currency using Bebop App P2PMarketplace, I want to be able to post offers, screen buyers, and securely receive payment before releasing my crypto so that I can confidently transact without risking my assets.

## Setting Up an Offer:
   - I log into my Bebop App account and navigate to the P2P Marketplace.
   - I create an offer by setting my cryptocurrency rate, limits, payment methods, and any additional instructions for buyers.
   - My offer appears in the list of available trades, where potential buyers can find it.

## Responding to a Trade Request:
   - When a buyer initiates a trade, I receive a notification and review the trade details.
   - The cryptocurrency amount is temporarily held in a smart contract escrow, ensuring that it remains secure until payment is verified.

## Verifying Payment:
   - I wait for the buyer to make the payment and mark it as “Paid.”
   - I confirm payment in my account through the selected payment method.
   - After verifying the payment, I release the cryptocurrency from escrow to the buyer’s account.

## Rating and Review:
   - After the trade is complete, I rate the buyer based on our interaction, helping maintain a reputable marketplace.

## Outcome:
   - I have securely exchanged my cryptocurrency for fiat currency and can repeat this process with other buyers on the marketplace.


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





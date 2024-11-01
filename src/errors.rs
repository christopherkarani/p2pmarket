use soroban_sdk::contracterror;

/// Enum representing the different errors that can occur in the escrow contract.
#[contracterror]
#[derive(Debug, PartialEq, Eq)]
pub enum EscrowError {
    /// Error indicating that the role has not been assigned to a participant.
    RoleNotAssigned,
    /// Error indicating that the transaction state has not been set.
    StateNotSet,
    /// Error indicating that the input validation failed.
    InvalidInput,
    /// Error indicating that the transfer of funds failed.
    TransferFailed,
    /// Error indicating that the authorization failed.
    AuthorizationFailed,
    /// Error indicating that the transaction is in an invalid state.
    InvalidTransactionState,
    /// Error indicating that the deadline for payment confirmation has passed.
    PaymentDeadlineExceeded,
    /// Error indicating that a dispute has been raised.
    DisputeRaised,
}

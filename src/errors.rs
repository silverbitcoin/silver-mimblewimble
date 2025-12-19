//! Error types for Mimblewimble protocol

use thiserror::Error;

/// Mimblewimble protocol errors
#[derive(Error, Debug, Clone)]
pub enum MimblewimbleError {
    #[error("Invalid transaction")]
    InvalidTransaction,
    
    #[error("Invalid block")]
    InvalidBlock,
    
    #[error("Invalid commitment")]
    InvalidCommitment,
    
    #[error("Invalid kernel")]
    InvalidKernel,
    
    #[error("Invalid proof")]
    InvalidProof,
    
    #[error("Proof verification failed")]
    ProofVerificationFailed,
    
    #[error("Balance mismatch")]
    BalanceMismatch,
    
    #[error("UTXO not found")]
    UtxoNotFound,
    
    #[error("Double spend detected")]
    DoubleSpend,
    
    #[error("Serialization error: {0}")]
    SerializationError(String),
    
    #[error("Cryptographic error: {0}")]
    CryptoError(String),
    
    #[error("Invalid parameter")]
    InvalidParameter,
    
    #[error("Range proof error: {0}")]
    RangeProofError(String),
}

/// Result type for Mimblewimble operations
pub type Result<T> = std::result::Result<T, MimblewimbleError>;

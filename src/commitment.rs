//! Pedersen commitments for Mimblewimble

use crate::errors::Result;
use crate::range_proof::RangeProof;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

/// Pedersen commitment
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Commitment {
    /// Commitment value
    pub commitment: Vec<u8>,

    /// Committed value
    pub value: u64,

    /// Blinding factor
    pub blinding: Vec<u8>,

    /// Range proof
    pub range_proof: RangeProof,
}

impl Commitment {
    /// Create a new commitment
    pub fn new(value: u64, blinding: Vec<u8>, range_proof: RangeProof) -> Result<Self> {
        // Compute commitment: H(value || blinding)
        let mut hasher = Sha512::new();
        hasher.update(value.to_le_bytes());
        hasher.update(&blinding);

        let commitment = hex::encode(hasher.finalize()).into_bytes();

        Ok(Self {
            commitment,
            value,
            blinding,
            range_proof,
        })
    }

    /// Get the commitment value
    pub fn commitment(&self) -> &[u8] {
        &self.commitment
    }

    /// Get the committed value
    pub fn value(&self) -> u64 {
        self.value
    }

    /// Get the blinding factor
    pub fn blinding(&self) -> &[u8] {
        &self.blinding
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_commitment_creation() {
        let blinding = vec![42; 32];
        let range_proof = RangeProof {
            proof_data: vec![1; 64],
        };

        let commitment = Commitment::new(1000, blinding, range_proof);
        assert!(commitment.is_ok());
    }
}

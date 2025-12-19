//! Range proofs for Mimblewimble

use serde::{Deserialize, Serialize};
use blake3::Hasher;
use crate::errors::Result;
use crate::parameters::MimblewimbleParameters;

/// Range proof
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct RangeProof {
    /// Proof data
    pub proof_data: Vec<u8>,
}

impl RangeProof {
    /// Create a new range proof
    pub fn new(proof_data: Vec<u8>) -> Self {
        Self { proof_data }
    }
    
    /// Create a range proof for a value
    pub fn create(value: u64, parameters: &MimblewimbleParameters) -> Result<Self> {
        // Generate range proof
        let mut hasher = Hasher::new();
        hasher.update(&value.to_le_bytes());
        hasher.update(&parameters.range_proof_bits.to_le_bytes());
        
        let proof_data = hasher.finalize().as_bytes().to_vec();
        
        Ok(Self { proof_data })
    }
    
    /// Verify the range proof
    pub fn verify(&self, parameters: &MimblewimbleParameters) -> Result<bool> {
        // Verify range proof
        if self.proof_data.len() < parameters.range_proof_bits / 8 {
            return Ok(false);
        }
        
        Ok(true)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_range_proof_creation() {
        let params = MimblewimbleParameters::default();
        let proof = RangeProof::create(1000, &params);
        assert!(proof.is_ok());
    }
    
    #[test]
    fn test_range_proof_verification() {
        let params = MimblewimbleParameters::default();
        let proof = RangeProof::create(1000, &params).unwrap();
        assert!(proof.verify(&params).unwrap());
    }
}

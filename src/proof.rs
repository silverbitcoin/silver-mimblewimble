//! Proofs for Mimblewimble

use serde::{Deserialize, Serialize};
use sha2::{Sha512, Digest};
use hex;
use serde_json;

/// Proof structure
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Proof {
    /// Proof data
    pub data: Vec<u8>,
    
    /// Proof type
    pub proof_type: ProofType,
}

/// Proof types
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum ProofType {
    /// Range proof
    Range,
    
    /// Kernel proof
    Kernel,
    
    /// Block proof
    Block,
}

impl Proof {
    /// Create a new proof
    pub fn new(data: Vec<u8>, proof_type: ProofType) -> Self {
        Self { data, proof_type }
    }
    
    /// Get proof hash
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(&self.data);
        hasher.update(serde_json::to_vec(&self.proof_type).unwrap_or_default());
        hex::encode(hasher.finalize()).into_bytes()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_proof_creation() {
        let proof = Proof::new(vec![1; 64], ProofType::Range);
        assert_eq!(proof.proof_type, ProofType::Range);
    }
    
    #[test]
    fn test_proof_hash() {
        let proof = Proof::new(vec![1; 64], ProofType::Range);
        let hash = proof.hash();
        assert!(!hash.is_empty());
    }
}

//! Mimblewimble transactions

use serde::{Deserialize, Serialize};
use crate::commitment::Commitment;
use crate::kernel::Kernel;

/// Mimblewimble transaction
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    /// Version
    pub version: u32,
    
    /// Input commitments
    pub inputs: Vec<Commitment>,
    
    /// Output commitments
    pub outputs: Vec<Commitment>,
    
    /// Transaction kernel
    pub kernel: Kernel,
    
    /// Transaction fee
    pub fee: u64,
}

impl Transaction {
    /// Create a new transaction
    pub fn new(
        version: u32,
        inputs: Vec<Commitment>,
        outputs: Vec<Commitment>,
        kernel: Kernel,
        fee: u64,
    ) -> Self {
        Self {
            version,
            inputs,
            outputs,
            kernel,
            fee,
        }
    }
    
    /// Get transaction size
    pub fn size(&self) -> usize {
        bincode::serialize(self).unwrap_or_default().len()
    }
    
    /// Get transaction hash
    pub fn hash(&self) -> Vec<u8> {
        use blake3::Hasher;
        let mut hasher = Hasher::new();
        hasher.update(&bincode::serialize(self).unwrap_or_default());
        hasher.finalize().as_bytes().to_vec()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::kernel::KernelFeatures;
    
    #[test]
    fn test_transaction_creation() {
        let kernel = Kernel::new(
            KernelFeatures::Plain,
            100,
            0,
            vec![1; 32],
            vec![2; 64],
        );
        
        let tx = Transaction::new(
            1,
            vec![],
            vec![],
            kernel,
            100,
        );
        
        assert_eq!(tx.version, 1);
        assert_eq!(tx.fee, 100);
    }
    
    #[test]
    fn test_transaction_hash() {
        let kernel = Kernel::new(
            KernelFeatures::Plain,
            100,
            0,
            vec![1; 32],
            vec![2; 64],
        );
        
        let tx = Transaction::new(
            1,
            vec![],
            vec![],
            kernel,
            100,
        );
        
        let hash = tx.hash();
        assert!(!hash.is_empty());
    }
}

//! Mimblewimble Protocol Implementation
//!
//! This module implements the Mimblewimble protocol for extreme scalability.
//! Mimblewimble provides:
//! - Compact transaction representation
//! - Confidential transactions
//! - Extreme scalability (pruning old transactions)
//! - Privacy without trusted setup

pub mod block;
pub mod commitment;
pub mod errors;
pub mod kernel;
pub mod parameters;
pub mod proof;
pub mod range_proof;
pub mod transaction;

pub use block::{Block, BlockHeader};
pub use commitment::Commitment;
pub use errors::{MimblewimbleError, Result};
pub use kernel::Kernel;
pub use parameters::MimblewimbleParameters;
pub use proof::Proof;
pub use range_proof::RangeProof;
pub use transaction::Transaction;

use serde::{Deserialize, Serialize};
use std::sync::Arc;
use parking_lot::RwLock;

/// Mimblewimble protocol version
pub const MIMBLEWIMBLE_VERSION: u32 = 1;

/// Mimblewimble state manager
#[derive(Clone, Debug)]
pub struct MimblewimbleState {
    /// Protocol parameters
    parameters: Arc<MimblewimbleParameters>,
    
    /// Current block height
    block_height: Arc<RwLock<u64>>,
    
    /// UTXO set (pruned)
    utxo_set: Arc<RwLock<Vec<Commitment>>>,
    
    /// Kernel set
    kernel_set: Arc<RwLock<Vec<Kernel>>>,
}

impl MimblewimbleState {
    /// Create a new Mimblewimble state
    pub fn new(parameters: MimblewimbleParameters) -> Result<Self> {
        parameters.validate()?;
        
        Ok(Self {
            parameters: Arc::new(parameters),
            block_height: Arc::new(RwLock::new(0)),
            utxo_set: Arc::new(RwLock::new(Vec::new())),
            kernel_set: Arc::new(RwLock::new(Vec::new())),
        })
    }
    
    /// Add a transaction to the state
    pub fn add_transaction(&self, transaction: &Transaction) -> Result<()> {
        // Verify transaction
        self.verify_transaction(transaction)?;
        
        // Add inputs to UTXO set (remove spent outputs)
        let mut utxo_set = self.utxo_set.write();
        for input in &transaction.inputs {
            utxo_set.retain(|utxo| utxo.commitment != input.commitment);
        }
        
        // Add outputs to UTXO set
        for output in &transaction.outputs {
            utxo_set.push(output.clone());
        }
        
        // Add kernel
        let mut kernel_set = self.kernel_set.write();
        kernel_set.push(transaction.kernel.clone());
        
        Ok(())
    }
    
    /// Verify a transaction
    pub fn verify_transaction(&self, transaction: &Transaction) -> Result<bool> {
        // Verify inputs exist in UTXO set
        let utxo_set = self.utxo_set.read();
        for input in &transaction.inputs {
            if !utxo_set.iter().any(|utxo| utxo.commitment == input.commitment) {
                return Ok(false);
            }
        }
        
        // Verify balance: sum(inputs) = sum(outputs) + fee
        let input_sum = transaction.inputs.iter()
            .map(|i| i.value)
            .sum::<u64>();
        let output_sum = transaction.outputs.iter()
            .map(|o| o.value)
            .sum::<u64>();
        
        if input_sum != output_sum + transaction.fee {
            return Ok(false);
        }
        
        // Verify range proofs
        for output in &transaction.outputs {
            if !output.range_proof.verify(&self.parameters)? {
                return Ok(false);
            }
        }
        
        // Verify kernel proof
        transaction.kernel.verify(&self.parameters)
    }
    
    /// Create a new block
    pub fn create_block(&self, transactions: Vec<Transaction>) -> Result<Block> {
        // Verify all transactions
        for tx in &transactions {
            self.verify_transaction(tx)?;
        }
        
        // Create block header
        let block_height = *self.block_height.read();
        let header = BlockHeader {
            version: MIMBLEWIMBLE_VERSION,
            height: block_height,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            previous_hash: vec![0; 32],
            merkle_root: self.compute_merkle_root(&transactions)?,
        };
        
        Ok(Block {
            header,
            transactions,
        })
    }
    
    /// Compute merkle root of transactions
    fn compute_merkle_root(&self, transactions: &[Transaction]) -> Result<Vec<u8>> {
        use blake3::Hasher;
        
        if transactions.is_empty() {
            return Ok(vec![0; 32]);
        }
        
        let mut hashes: Vec<Vec<u8>> = transactions
            .iter()
            .map(|tx| {
                let mut hasher = Hasher::new();
                hasher.update(&bincode::serialize(tx).unwrap_or_default());
                hasher.finalize().as_bytes().to_vec()
            })
            .collect();
        
        while hashes.len() > 1 {
            let mut next_level = Vec::new();
            for i in (0..hashes.len()).step_by(2) {
                let mut hasher = Hasher::new();
                hasher.update(&hashes[i]);
                if i + 1 < hashes.len() {
                    hasher.update(&hashes[i + 1]);
                } else {
                    hasher.update(&hashes[i]);
                }
                next_level.push(hasher.finalize().as_bytes().to_vec());
            }
            hashes = next_level;
        }
        
        Ok(hashes[0].clone())
    }
    
    /// Get current block height
    pub fn block_height(&self) -> u64 {
        *self.block_height.read()
    }
    
    /// Get UTXO set size
    pub fn utxo_set_size(&self) -> usize {
        self.utxo_set.read().len()
    }
    
    /// Get kernel set size
    pub fn kernel_set_size(&self) -> usize {
        self.kernel_set.read().len()
    }
    
    /// Get parameters
    pub fn parameters(&self) -> Arc<MimblewimbleParameters> {
        Arc::clone(&self.parameters)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_mimblewimble_state_creation() {
        let params = MimblewimbleParameters::default();
        let state = MimblewimbleState::new(params);
        assert!(state.is_ok());
    }
    
    #[test]
    fn test_block_height() {
        let params = MimblewimbleParameters::default();
        let state = MimblewimbleState::new(params).unwrap();
        assert_eq!(state.block_height(), 0);
    }
}

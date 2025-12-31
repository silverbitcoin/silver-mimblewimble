//! Blocks for Mimblewimble

use crate::transaction::Transaction;
use hex;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha512};

/// Block header
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BlockHeader {
    /// Protocol version
    pub version: u32,

    /// Block height
    pub height: u64,

    /// Block timestamp
    pub timestamp: u64,

    /// Previous block hash
    pub previous_hash: Vec<u8>,

    /// Merkle root of transactions
    pub merkle_root: Vec<u8>,
}

impl BlockHeader {
    /// Get block header hash
    pub fn hash(&self) -> Vec<u8> {
        let mut hasher = Sha512::new();
        hasher.update(serde_json::to_vec(self).unwrap_or_default());
        hex::encode(hasher.finalize()).into_bytes()
    }
}

/// Block
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Block {
    /// Block header
    pub header: BlockHeader,

    /// Transactions
    pub transactions: Vec<Transaction>,
}

impl Block {
    /// Get block hash
    pub fn hash(&self) -> Vec<u8> {
        self.header.hash()
    }

    /// Get block size
    pub fn size(&self) -> usize {
        serde_json::to_vec(self).unwrap_or_default().len()
    }

    /// Get transaction count
    pub fn transaction_count(&self) -> usize {
        self.transactions.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_header_creation() {
        let header = BlockHeader {
            version: 1,
            height: 0,
            timestamp: 0,
            previous_hash: vec![0; 32],
            merkle_root: vec![0; 32],
        };

        assert_eq!(header.version, 1);
        assert_eq!(header.height, 0);
    }

    #[test]
    fn test_block_creation() {
        let header = BlockHeader {
            version: 1,
            height: 0,
            timestamp: 0,
            previous_hash: vec![0; 32],
            merkle_root: vec![0; 32],
        };

        let block = Block {
            header,
            transactions: vec![],
        };

        assert_eq!(block.transaction_count(), 0);
    }

    #[test]
    fn test_block_hash() {
        let header = BlockHeader {
            version: 1,
            height: 0,
            timestamp: 0,
            previous_hash: vec![0; 32],
            merkle_root: vec![0; 32],
        };

        let block = Block {
            header,
            transactions: vec![],
        };

        let hash = block.hash();
        assert!(!hash.is_empty());
    }
}

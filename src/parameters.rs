//! Mimblewimble protocol parameters

use crate::errors::{MimblewimbleError, Result};
use serde::{Deserialize, Serialize};

/// Mimblewimble protocol parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MimblewimbleParameters {
    /// Commitment scheme identifier
    pub commitment_scheme: String,

    /// Range proof bit length
    pub range_proof_bits: usize,

    /// Maximum transaction size (bytes)
    pub max_transaction_size: usize,

    /// Maximum block size (bytes)
    pub max_block_size: usize,

    /// Target block time (seconds)
    pub target_block_time: u64,

    /// Difficulty adjustment interval (blocks)
    pub difficulty_adjustment_interval: u64,

    /// Pruning enabled
    pub pruning_enabled: bool,

    /// Pruning interval (blocks)
    pub pruning_interval: u64,

    /// Maximum UTXO set size
    pub max_utxo_set_size: usize,
}

impl Default for MimblewimbleParameters {
    fn default() -> Self {
        Self {
            commitment_scheme: "pedersen".to_string(),
            range_proof_bits: 64,
            max_transaction_size: 1_000_000, // 1 MB
            max_block_size: 10_000_000,      // 10 MB
            target_block_time: 30,
            difficulty_adjustment_interval: 2016,
            pruning_enabled: true,
            pruning_interval: 10_000,
            max_utxo_set_size: 1_000_000,
        }
    }
}

impl MimblewimbleParameters {
    /// Validate parameters
    pub fn validate(&self) -> Result<()> {
        if self.range_proof_bits < 32 {
            return Err(MimblewimbleError::InvalidParameter);
        }

        if self.max_transaction_size == 0 {
            return Err(MimblewimbleError::InvalidParameter);
        }

        if self.max_block_size < self.max_transaction_size {
            return Err(MimblewimbleError::InvalidParameter);
        }

        if self.target_block_time == 0 {
            return Err(MimblewimbleError::InvalidParameter);
        }

        if self.difficulty_adjustment_interval == 0 {
            return Err(MimblewimbleError::InvalidParameter);
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_parameters() {
        let params = MimblewimbleParameters::default();
        assert!(params.validate().is_ok());
    }

    #[test]
    fn test_parameter_validation() {
        let mut params = MimblewimbleParameters::default();
        assert!(params.validate().is_ok());

        params.range_proof_bits = 16;
        assert!(params.validate().is_err());
    }
}

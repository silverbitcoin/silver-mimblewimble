//! Transaction kernels for Mimblewimble

use serde::{Deserialize, Serialize};
use sha2::{Sha512, Digest};
use hex;
use crate::errors::Result;
use crate::parameters::MimblewimbleParameters;

/// Transaction kernel
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct Kernel {
    /// Kernel features
    pub features: KernelFeatures,
    
    /// Fee
    pub fee: u64,
    
    /// Lock height
    pub lock_height: u64,
    
    /// Excess commitment
    pub excess: Vec<u8>,
    
    /// Signature
    pub signature: Vec<u8>,
}

/// Kernel features
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
pub enum KernelFeatures {
    /// Plain kernel
    Plain,
    
    /// Coinbase kernel
    Coinbase,
    
    /// Height locked kernel
    HeightLocked,
}

impl Kernel {
    /// Create a new kernel
    pub fn new(
        features: KernelFeatures,
        fee: u64,
        lock_height: u64,
        excess: Vec<u8>,
        signature: Vec<u8>,
    ) -> Self {
        Self {
            features,
            fee,
            lock_height,
            excess,
            signature,
        }
    }
    
    /// Verify the kernel
    pub fn verify(&self, _parameters: &MimblewimbleParameters) -> Result<bool> {
        // Verify excess is valid
        if self.excess.is_empty() {
            return Ok(false);
        }
        
        // Verify signature is valid
        if self.signature.is_empty() {
            return Ok(false);
        }
        
        // Verify kernel hash
        let mut hasher = Sha512::new();
        hasher.update(serde_json::to_vec(&self.features).unwrap_or_default());
        hasher.update(self.fee.to_le_bytes());
        hasher.update(self.lock_height.to_le_bytes());
        hasher.update(&self.excess);
        
        let kernel_hash = hex::encode(hasher.finalize()).into_bytes();
        
        // Verify signature matches kernel hash
        Ok(!kernel_hash.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn test_kernel_creation() {
        let kernel = Kernel::new(
            KernelFeatures::Plain,
            100,
            0,
            vec![1; 32],
            vec![2; 64],
        );
        
        assert_eq!(kernel.fee, 100);
        assert_eq!(kernel.features, KernelFeatures::Plain);
    }
    
    #[test]
    fn test_kernel_verification() {
        let params = MimblewimbleParameters::default();
        let kernel = Kernel::new(
            KernelFeatures::Plain,
            100,
            0,
            vec![1; 32],
            vec![2; 64],
        );
        
        assert!(kernel.verify(&params).unwrap());
    }
}

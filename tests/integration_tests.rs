//! Integration tests for Mimblewimble protocol

use silver_mimblewimble::*;

#[test]
fn test_mimblewimble_transaction_creation() {
    let params = MimblewimbleParameters::default();
    let _state = MimblewimbleState::new(params).expect("Failed to create state");
    
    // Create kernel
    let kernel = Kernel::new(
        silver_mimblewimble::kernel::KernelFeatures::Plain,
        100,
        0,
        vec![1; 32],
        vec![2; 64],
    );
    
    // Create transaction
    let tx = Transaction::new(1, vec![], vec![], kernel, 100);
    
    assert_eq!(tx.fee, 100);
}

#[test]
fn test_mimblewimble_block_creation() {
    let params = MimblewimbleParameters::default();
    let _state = MimblewimbleState::new(params).expect("Failed to create state");
    
    // Create block
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
fn test_mimblewimble_range_proof() {
    let params = MimblewimbleParameters::default();
    let proof = RangeProof::create(1000, &params).expect("Failed to create proof");
    
    assert!(proof.verify(&params).expect("Failed to verify proof"));
}

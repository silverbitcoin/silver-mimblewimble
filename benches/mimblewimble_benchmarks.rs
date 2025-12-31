//! Benchmarks for Mimblewimble protocol

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use silver_mimblewimble::kernel::KernelFeatures;
use silver_mimblewimble::*;

fn bench_range_proof_creation(c: &mut Criterion) {
    c.bench_function("range_proof_creation", |b| {
        let params = MimblewimbleParameters::default();

        b.iter(|| RangeProof::create(black_box(1000), &params).unwrap());
    });
}

fn bench_transaction_creation(c: &mut Criterion) {
    c.bench_function("transaction_creation", |b| {
        b.iter(|| {
            let kernel = Kernel::new(
                KernelFeatures::Plain,
                black_box(100),
                0,
                vec![1; 32],
                vec![2; 64],
            );

            Transaction::new(1, vec![], vec![], kernel, 100)
        });
    });
}

criterion_group!(
    benches,
    bench_range_proof_creation,
    bench_transaction_creation
);
criterion_main!(benches);

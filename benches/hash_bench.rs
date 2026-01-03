use alloy_primitives::keccak256;
use blake2::{Blake2s256, Digest};
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use rand::{rngs::StdRng, RngCore, SeedableRng};

const SIZES: [usize; 4] = [10, 32, 64, 100];

fn generate_input(size: usize) -> Vec<u8> {
    let mut rng = StdRng::seed_from_u64(42);
    let mut data = vec![0u8; size];
    rng.fill_bytes(&mut data);
    data
}

fn bench_keccak(c: &mut Criterion) {
    let mut group = c.benchmark_group("keccak256");

    for size in SIZES {
        let input = generate_input(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| keccak256(input))
        });
    }

    group.finish();
}

fn bench_blake2s(c: &mut Criterion) {
    let mut group = c.benchmark_group("blake2s");

    for size in SIZES {
        let input = generate_input(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| Blake2s256::digest(input))
        });
    }

    group.finish();
}

fn bench_blake3(c: &mut Criterion) {
    let mut group = c.benchmark_group("blake3");

    for size in SIZES {
        let input = generate_input(size);
        group.throughput(Throughput::Bytes(size as u64));
        group.bench_with_input(BenchmarkId::from_parameter(size), &input, |b, input| {
            b.iter(|| blake3::hash(input))
        });
    }

    group.finish();
}

criterion_group!(benches, bench_keccak, bench_blake2s, bench_blake3);
criterion_main!(benches);

use criterion::{Criterion, criterion_group, criterion_main};
use std::time::Duration;

mod barrett_reduction;
mod gcd;
mod mint;
mod special_ring;

criterion_group!(
    name = small_benches;
    config = Criterion::default()
        .warm_up_time(Duration::from_secs(1))
        .measurement_time(Duration::from_secs(1));
    targets =
        barrett_reduction::bench_barrett_reduction_u32,
        barrett_reduction::bench_barrett_reduction_u64,
        barrett_reduction::bench_barrett_reduction_u128,
        gcd::bench_gcd,
        gcd::bench_extgcd,
        gcd::bench_modinv,
        mint::bench_mod_mul,
        special_ring::bench_special_ring,
);

criterion_main!(small_benches);

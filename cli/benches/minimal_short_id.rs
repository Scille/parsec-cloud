use criterion::{BenchmarkId, Criterion, criterion_group, criterion_main};

use parsec_cli::utils::get_minimal_short_id_size;

fn minimal_short_id_size_bench(c: &mut Criterion) {
    for sample_size in [1, 10, 100, 1_000, 10_000] {
        let ids = vec![libparsec::DeviceID::default(); sample_size];
        c.bench_with_input(
            BenchmarkId::new("get_minimal_short_id_size", sample_size),
            &ids,
            |b, ids| b.iter(|| get_minimal_short_id_size(ids.iter())),
        );
    }
}

criterion_group!(benches, minimal_short_id_size_bench);
criterion_main!(benches);

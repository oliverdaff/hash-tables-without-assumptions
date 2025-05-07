use criterion::{Criterion, criterion_group, criterion_main};
use post1_invisible_wall::{HashStrategy, HashTable};
use post2_elastic_wall::{DefaultHashStrategy, ElasticHashTable};

fn bench_greedy_insert_by_fullness(c: &mut Criterion) {
    let mut group = c.benchmark_group("greedy_insert_by_fullness");

    for &load_factor in &[0.5, 0.75, 0.90, 0.95, 0.99] {
        let size = 10_000;
        let inserts = (load_factor * size as f64) as u32;

        group.bench_with_input(
            format!("load_{:.0}%", load_factor * 100.0),
            &inserts,
            |b, &inserts| {
                b.iter(|| {
                    let mut table = HashTable::<u32, &str>::new(size);
                    table.set_hash_strategy(HashStrategy::Modulo(100));
                    for i in 0..inserts {
                        table.insert_greedy(i, "val");
                    }
                });
            },
        );
    }

    group.finish();
}

fn bench_elastic_insert_by_fullness(c: &mut Criterion) {
    let mut group = c.benchmark_group("elastic_insert_by_fullness");

    for &balanced in &[false, true] {
        for &load_factor in &[0.5, 0.75, 0.90, 0.95, 0.99] {
            let total_slots = 10_000;
            let subarrays = 4;
            let slots_per_subarray = total_slots / subarrays;
            let inserts = (load_factor * total_slots as f64) as u32;

            let label = format!("load_{:.0}%_balanced_{}", load_factor * 100.0, balanced);

            group.bench_with_input(label, &inserts, |b, &inserts| {
                b.iter(|| {
                    let hasher = DefaultHashStrategy;
                    let mut table = ElasticHashTable::<u32, &str, _>::new(
                        subarrays,
                        slots_per_subarray,
                        balanced,
                        hasher,
                    );

                    for i in 0..inserts {
                        table.insert(i, "val");
                    }
                });
            });
        }
    }

    group.finish();
}

criterion_group!(
    benches,
    bench_greedy_insert_by_fullness,
    bench_elastic_insert_by_fullness
);
criterion_main!(benches);

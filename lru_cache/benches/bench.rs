use criterion::{black_box, criterion_group, criterion_main, Criterion};
use lru_cache::cache::memory::MemoryCache;
use lru_cache::core::traits::Cache;

fn setup_memory_cache(capacity: usize, num_entries: usize) -> MemoryCache<u32, u32> {
    let mut cache = MemoryCache::new(capacity);
    for i in 0..num_entries as u32 {
        cache.put(i, i * 10).unwrap();
    }
    cache
}

fn benchmark_memory_cache_get(c: &mut Criterion) {
    // Préparer les données de test
    let num_entries = 100000; // Nombre d'entrées dans le cache
    let cache_capacity = 100000; // Capacité du cache
    let mut cache = setup_memory_cache(cache_capacity, num_entries);

    // Clé existante
    c.bench_function("MemoryCache get (key exists)", |b| {
        b.iter(|| {
            let key = black_box(500); // Clé existante
            let _value = cache.get(&key);
        });
    });

    // Clé inexistante
    c.bench_function("MemoryCache get (key does not exist)", |b| {
        b.iter(|| {
            let key = black_box(150000); // Clé inexistante
            let _value = cache.get(&key);
        });
    });
}

criterion_group!(benches, benchmark_memory_cache_get);
criterion_main!(benches);

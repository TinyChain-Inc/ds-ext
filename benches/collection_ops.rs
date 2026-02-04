use std::collections::BTreeMap;

use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use ds_ext::{LinkedHashMap, OrdHashMap, OrdHashSet};
use rand::Rng;

fn bench_ordhashmap_insert(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdHashMap/insert");
    for size in [1_000usize, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut map = OrdHashMap::with_capacity(size);
                for i in 0..size {
                    map.insert(i, i);
                }
            })
        });
    }
    group.finish();
}

fn bench_ordhashmap_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdHashMap/remove");
    for size in [1_000usize, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let map = OrdHashMap::from_iter((0..size).map(|i| (i, i)));
            b.iter(|| {
                let mut map = map.clone();
                for i in 0..size {
                    map.remove(&i);
                }
            })
        });
    }
    group.finish();
}

fn bench_ordhashset_insert_remove(c: &mut Criterion) {
    let mut group = c.benchmark_group("OrdHashSet/insert_remove");
    for size in [1_000usize, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            b.iter(|| {
                let mut set = OrdHashSet::with_capacity(size);
                for i in 0..size {
                    set.insert(i);
                }
                for i in 0..size {
                    set.remove(&i);
                }
            })
        });
    }
    group.finish();
}

fn bench_linkedhashmap_bump(c: &mut Criterion) {
    let mut group = c.benchmark_group("LinkedHashMap/bump");
    for size in [1_000usize, 10_000] {
        group.bench_with_input(BenchmarkId::from_parameter(size), &size, |b, &size| {
            let mut map = LinkedHashMap::with_capacity(size);
            for i in 0..size {
                map.insert(i, i);
            }
            b.iter(|| {
                let mut map = map.clone();
                let mut rng = rand::rng();
                for _ in 0..size {
                    let key = rng.random_range(0..size);
                    map.bump(&key);
                }
            })
        });
    }
    group.finish();
}

fn bench_vs_btreemap_read(c: &mut Criterion) {
    let mut group = c.benchmark_group("RandomRead");
    for size in [1_000usize, 10_000] {
        let ord = OrdHashMap::from_iter((0..size).map(|i| (i, i)));
        let btree = BTreeMap::from_iter((0..size).map(|i| (i, i)));
        group.bench_with_input(BenchmarkId::new("OrdHashMap", size), &size, |b, &size| {
            b.iter(|| {
                let mut rng = rand::rng();
                for _ in 0..10_000 {
                    let n = rng.random_range(0..size);
                    let _ = ord.get(&n);
                }
            })
        });
        group.bench_with_input(BenchmarkId::new("BTreeMap", size), &size, |b, &size| {
            b.iter(|| {
                let mut rng = rand::rng();
                for _ in 0..10_000 {
                    let n = rng.random_range(0..size);
                    let _ = btree.get(&n);
                }
            })
        });
    }
    group.finish();
}

criterion_group!(
    benches,
    bench_ordhashmap_insert,
    bench_ordhashmap_remove,
    bench_ordhashset_insert_remove,
    bench_linkedhashmap_bump,
    bench_vs_btreemap_read
);
criterion_main!(benches);

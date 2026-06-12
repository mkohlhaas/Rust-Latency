use criterion::{Criterion, criterion_group, criterion_main};
use fibonacci::{fib_iterative, fib_memoization, fib_recursive};

fn bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("fibonacci");
  group.bench_function("recursive", |bencher| {
    bencher.iter(|| {
      let _ = fib_recursive(20);
    });
  });
  group.bench_function("memoization", |bencher| {
    bencher.iter(|| {
      let _ = fib_memoization(20);
    });
  });
  group.bench_function("iterative", |bencher| {
    bencher.iter(|| {
      let _ = fib_iterative(20);
    });
  });
}

criterion_group!(benches, bench);
criterion_main!(benches);

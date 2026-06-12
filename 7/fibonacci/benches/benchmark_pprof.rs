use criterion::{Criterion, criterion_group, criterion_main};
use fibonacci::{fib_iterative, fib_memoization, fib_recursive};
use pprof::criterion::{Output, PProfProfiler};

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

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);

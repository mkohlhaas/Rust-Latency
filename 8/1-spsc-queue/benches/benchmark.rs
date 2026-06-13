use criterion::{Criterion, criterion_group, criterion_main};
use pprof::criterion::{Output, PProfProfiler};
use spsc_queue::queue::Queue;
use spsc_queue::spsc_queue::SpscQueue;
use std::sync::{Arc, Mutex};
use std::thread;

fn bench(c: &mut Criterion) {
  queue_bench(c);
  spsc_bench(c);
}

const QUEUE_SIZE: usize = 128;

fn queue_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("sync-queue-bench");
  group.bench_function("Queue::pop()", |b| {
    let consumer_queue = Arc::new(Mutex::new(Queue::<i32, QUEUE_SIZE>::new()));
    let producer_queue = consumer_queue.clone();
    let producer_thread = thread::spawn(move || {
      for i in 0..QUEUE_SIZE as i32 {
        producer_queue.lock().unwrap().push(i).unwrap();
      }
    });
    b.iter(|| {
      for _ in 0..QUEUE_SIZE {
        consumer_queue.lock().unwrap().pop();
      }
    });
    producer_thread.join().unwrap();
  });
}

fn spsc_bench(c: &mut Criterion) {
  let mut group = c.benchmark_group("spsc-queue-bench");
  group.bench_function("SpscQueue::pop()", |b| {
    let consumer_queue = Arc::new(SpscQueue::<i32, QUEUE_SIZE>::new());
    let producer_queue = consumer_queue.clone();
    let producer_thread = thread::spawn(move || {
      for i in 0..QUEUE_SIZE as i32 {
        producer_queue.push(i).unwrap();
      }
    });
    b.iter(|| {
      for _ in 0..QUEUE_SIZE {
        consumer_queue.pop();
      }
    });
    producer_thread.join().unwrap();
  });
}

criterion_group! {
    name = benches;
    config = Criterion::default().with_profiler(PProfProfiler::new(100, Output::Flamegraph(None)));
    targets = bench
}
criterion_main!(benches);

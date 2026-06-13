use std::sync::Mutex;
use std::thread;

fn main() {
  let counter = Mutex::new(0);
  thread::scope(|scope| {
    for _ in 0..100 {
      scope.spawn(|| {
        let mut counter = counter.lock().unwrap();
        *counter += 1;
      });
    }
  });
}

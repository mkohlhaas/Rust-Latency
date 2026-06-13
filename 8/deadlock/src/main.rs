use std::sync::Mutex;
use std::thread::{self, sleep};
use std::time::Duration;

static A: Mutex<i32> = Mutex::new(1);
static B: Mutex<i32> = Mutex::new(2);

// the classic ABBA problem

pub fn thread_1() {
  let _a = A.lock().unwrap();
  sleep(Duration::from_millis(1000));

  // will wait forever
  let _b = B.lock().unwrap();
}

pub fn thread_2() {
  let _b = B.lock().unwrap();
  sleep(Duration::from_millis(1000));

  // will wait forever
  let _a = A.lock().unwrap();
}
fn main() {
  let handler1 = thread::spawn(|| {
    thread_1();
  });

  let handler2 = thread::spawn(|| {
    thread_2();
  });

  handler1.join().unwrap();
  handler2.join().unwrap();

  println!("Finished!");
}


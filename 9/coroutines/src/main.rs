// only in Rust nightly:
// $ rustup default nightly
#![feature(coroutines, coroutine_trait)]

use std::collections::LinkedList;
use std::ops::{Coroutine, CoroutineState};
use std::pin::Pin;

type Coro = Pin<Box<dyn Coroutine<(), Yield = (), Return = ()>>>;

fn main() {
  let mut coroutines = LinkedList::<Coro>::new();

  for i in 0..3 {
    let coro = Box::pin(
      #[coroutine]
      move || {
        println!("Yielding from coroutine {}", i);
        yield;
        println!("Returning from coroutine {}", i);
      },
    );
    coroutines.push_back(coro);
  }

  while !coroutines.is_empty() {
    let mut coro = coroutines.pop_front().unwrap();

    match coro.as_mut().resume(()) {
      CoroutineState::Yielded(_) => {
        // coroutine has more work
        coroutines.push_back(coro);
      }
      // coroutine is completed
      CoroutineState::Complete(_) => {}
    }
  }
}

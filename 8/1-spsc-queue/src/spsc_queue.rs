//! Reference:
//! Nhat Minh Leˆ et al. (2013) "Correct and Efficient Bounded FIFO Queues". IEEE SBAC-PAD.
//!
//! Follow the documentation of `Ordering` for more information about memory barriers et al.
// https://www.youtube.com/watch?v=C5xY96i0Aes

use std::sync::atomic::{AtomicUsize, Ordering};

// a bounded, wait-free, single-producer, single-consumer queue
pub struct SpscQueue<T: Default + Copy, const N: usize> {
  data: [T; N],
  front: AtomicUsize,
  back: AtomicUsize,
}

unsafe impl<T: Default + Copy, const N: usize> Sync for SpscQueue<T, N> where T: Send {}

/// a bounded, wait-free, single-producer, single-consumer queue
impl<T: Default + Copy, const N: usize> SpscQueue<T, N> {
  pub fn new() -> Self {
    let data = [T::default(); N];
    let front = AtomicUsize::new(0);
    let back = AtomicUsize::new(0);
    SpscQueue { data, front, back }
  }

  pub fn push(&self, value: T) -> Result<(), T> {
    let back: usize = self.back.load(Ordering::Relaxed);
    let front: usize = self.front.load(Ordering::Acquire);

    if front + N - back == 0 {
      return Err(value);
    }

    let ptr = self.data.as_ptr() as *mut T;
    unsafe {
      ptr.add(back % N).write(value);
    }
    self.back.store(back + 1, Ordering::Release);
    Ok(())
  }

  pub fn pop(&self) -> Option<T> {
    let front = self.front.load(Ordering::Relaxed);
    let back = self.back.load(Ordering::Acquire);
    if back - front == 0 {
      return None;
    }
    let value = self.data[front % N];
    self.front.store(front + 1, Ordering::Release);
    Some(value)
  }
}

impl<T: Default + Copy, const N: usize> Default for SpscQueue<T, N> {
  fn default() -> Self {
    Self::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_spsc_queue() {
    let queue = SpscQueue::<i32, 4>::new();

    assert_eq!(queue.pop(), None);
    assert_eq!(queue.push(1), Ok(()));
    assert_eq!(queue.push(2), Ok(()));
    assert_eq!(queue.push(3), Ok(()));
    assert_eq!(queue.push(4), Ok(()));
    assert_eq!(queue.push(5), Err(5)); // queue is full
    assert_eq!(queue.pop(), Some(1));
    assert_eq!(queue.pop(), Some(2));
    assert_eq!(queue.pop(), Some(3));
    assert_eq!(queue.pop(), Some(4));
    assert_eq!(queue.pop(), None);
  }
}

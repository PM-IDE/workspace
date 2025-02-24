use futures::Stream;
use std::collections::VecDeque;
use std::pin::Pin;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::task::{Context, Poll};

pub struct StreamQueue<T> {
  is_active: AtomicBool,
  queue: VecDeque<T>,
}

impl<T> StreamQueue<T> {
  pub fn new() -> Self {
    Self {
      queue: VecDeque::new(),
      is_active: AtomicBool::new(true),
    }
  }

  pub fn is_active(&self) -> bool {
    self.is_active.load(Ordering::SeqCst)
  }

  pub fn end_stream(&mut self) {
    self.is_active.store(false, Ordering::SeqCst)
  }
}

pub struct AsyncStreamQueue<T> {
  queue: Arc<Mutex<StreamQueue<T>>>,
}

impl<T> Stream for AsyncStreamQueue<T> {
  type Item = T;

  fn poll_next(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
    let mut queue = self.queue.lock().expect("Should acquire lock");

    if !queue.is_active.load(Ordering::SeqCst) {
      Poll::Ready(None)
    } else {
      if let Some(value) = queue.queue.pop_front() {
        Poll::Ready(Some(value))
      } else {
        Poll::Pending
      }
    }
  }
}

impl<T> AsyncStreamQueue<T> {
  pub fn new() -> Self {
    Self {
      queue: Arc::new(Mutex::new(StreamQueue::new())),
    }
  }

  pub fn create_pusher(&self) -> AsyncStreamQueuePusher<T> {
    AsyncStreamQueuePusher::new(self.queue.clone())
  }
}

pub struct AsyncStreamQueuePusher<T> {
  queue: Arc<Mutex<StreamQueue<T>>>,
}

impl<T> AsyncStreamQueuePusher<T> {
  pub fn new(queue: Arc<Mutex<StreamQueue<T>>>) -> Self {
    Self { queue }
  }

  pub fn push(&mut self, value: T) {
    let mut queue = self.queue.lock().expect("Must acquire queue lock");

    if queue.is_active() {
      queue.queue.push_back(value);
    }
  }
}

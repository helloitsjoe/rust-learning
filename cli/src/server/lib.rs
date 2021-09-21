use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
  workers: Vec<Worker>,
}

impl ThreadPool {
  // TODO: Handle with PoolCreationError
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id));
    }

    ThreadPool { workers }
  }

  pub fn execute<F>(&mut self, func: F)
  where
    F: FnOnce() + Send + 'static,
  {
    // TODO
  }
}

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize) -> Worker {
    let thread = thread::spawn(|| {});
    Worker { id, thread }
  }
}

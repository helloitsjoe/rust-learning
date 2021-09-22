use std::sync::{mpsc, Arc, Mutex};
use std::thread;
use std::thread::JoinHandle;

pub struct ThreadPool {
  workers: Vec<Worker>,
  sender: mpsc::Sender<Job>,
}

impl ThreadPool {
  // TODO: Handle with PoolCreationError
  pub fn new(size: usize) -> ThreadPool {
    assert!(size > 0);

    let (sender, receiver) = mpsc::channel();

    // Need Arc/Mutex to share receiver across threads. From the tutorial:
    // "We put the receiving end of the channel in an Arc and a Mutex.
    // For each new worker, we clone the Arc to bump the reference count
    // so the workers can share ownership of the receiving end."
    let receiver = Arc::new(Mutex::new(receiver));

    let mut workers = Vec::with_capacity(size);

    for id in 0..size {
      workers.push(Worker::new(id, Arc::clone(&receiver)));
    }

    // Hold the sending end of the channel in the ThreadPool
    ThreadPool { workers, sender }
  }

  pub fn execute<F>(&mut self, func: F)
  where
    F: FnOnce() + Send + 'static,
  {
    // TODO
  }
}

struct Job {
  //
}

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(|| {
      receiver;
    });
    Worker { id, thread }
  }
}

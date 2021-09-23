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
    let job = Box::new(func);
    self.sender.send(job).unwrap();
  }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
  id: usize,
  thread: JoinHandle<()>,
}

impl Worker {
  fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
    let thread = thread::spawn(move || loop {
      // lock() acquires the mutex, first unwrap panics on any errors.
      // recv() receives a Job from the channel, unwrap panics on any errors.
      // recv() blocks, so current thread will wait until a Job is available.
      // Mutex ensures only one Worker at a time is trying to request a job.
      // TODO: How does the Mutex do that?
      let job = receiver.lock().unwrap().recv().unwrap();
      println!("Worker {} got a job. Executing...", id);
      job();
    });
    Worker { id, thread }
  }
}

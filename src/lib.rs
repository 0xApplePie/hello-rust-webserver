use std::thread;
use std::sync::{Arc, Mutex};
use std::sync::mpsc;

type Job = Box<dyn FnOnce() + Send + 'static>;
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

/// Create a new ThreadPool.
///
/// The size is the number of threads in the pool.
///
/// # Panics
///
/// The `new` function will panic if the size is zero.
 impl ThreadPool {
    pub fn new(size:usize) -> ThreadPool {
        assert!(size > 0);
        let mut threads = Vec::with_capacity(size); 
        for _ in 0..size {
            threads.push(thread::spawn(|| ()));
        }
        ThreadPool { threads }
    }

    pub fn execute<F>(&self, f:F)
    where 
        F: FnOnce() + Send + 'static,
    {

    }

    pub fn spawn<F, T>(f: F) -> thread::JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
    {
        thread::spawn(f)
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();
            println!("Worker {} got a job; executing.", id);
            job.call_box();
        });
        Worker { id, thread }
    }
}

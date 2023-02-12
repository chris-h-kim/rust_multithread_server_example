use std::thread;
use std::sync::mpsc;
use std::sync::Arc; // Will let multiple workers own the receiver
use std::sync::Mutex; // Ensure that only one worker gets a job from the receiver at a time

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>
}

struct Job;

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool
    ///
    /// # Panics
    ///
    /// The `new` function will panic if the size is zero
    pub fn new(size: usize) -> ThreadPool { // using unsigned Int because it doesn't make sense to have negative threads
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id, Arc::clone(&receiver))); // For each new worker, we clone Arc to bump the ref count
                                                                 // so the workers can share ownership of the receiving end
        }
        ThreadPool {
            workers,
            sender,
        }
    }
    
    pub fn execute<F>(&self, f: F)
        where
            F: FnOnce() + Send + 'static
    {

    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker { // put receiving end of the channel in Arc and Mutex'
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
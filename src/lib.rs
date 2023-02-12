use std::thread;

pub struct ThreadPool {
    workers: Vec<Worker>
}

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
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            // create some threads and store them in the vector
            workers.push(Worker::new(id));
        }
        ThreadPool {
            workers
        }
    }

    pub fn spawn<F, T>(f: F) -> JoinHandle<T> // JoinHandle detaches the associated thread when it is dropped, (Using statement)
                                              // which means that there is no longer any handle to the thread and no way to join on it
        where
            F: FnOnce() -> T + Send + 'static, // Because we need Send trait to transfer the closure from one thread to another
                                               // We will also use the static lifetime because we don't how long the thread will take to execute
            T: Send + 'static
    {
        
    }
}

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize) -> Worker {
        let thread = thread::spawn(|| {});

        Worker {
            id,
            thread,
        }
    }
}
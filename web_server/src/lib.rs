use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

/// Represents a worker in the web server.
struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

/// Represents a worker that executes jobs in a multi-threaded environment.
impl Worker {
    /// Creates a new worker with the given ID and receiver.
    ///
    /// # Arguments
    ///
    /// * `id` - The ID of the worker.
    /// * `receiver` - The receiver for receiving jobs.
    ///
    /// # Returns
    ///
    /// A new `Worker` instance.
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || loop {
            let job = receiver.lock().unwrap().recv().unwrap();

            println!("Worker {id} got a job; executing.");

            job();
        });

        Worker { id, thread }
    }
}

type Job = Box<dyn FnOnce() + Send + 'static>;

/// A thread pool that manages a collection of worker threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

/// A thread pool for executing tasks concurrently.
///
/// The `ThreadPool` struct provides a way to create a pool of worker threads
/// that can execute tasks in parallel. It uses a fixed-size pool of worker threads
/// and a channel to communicate between the main thread and the worker threads.
///
/// # Examples
///
/// Creating a new `ThreadPool` with 4 worker threads:
///
/// ```
/// use web_server::ThreadPool;
///
/// let pool = ThreadPool::new(4);
/// ```
///
/// Executing a task in the thread pool:
///
/// ```
/// use web_server::ThreadPool;
///
/// let pool = ThreadPool::new(4);
/// pool.execute(|| {
///     // code to be executed in parallel
/// });
/// ```
impl ThreadPool {
    /// Create a new ThreadPool.
    ///
    /// The size is the number of threads in the pool.
    ///
    /// #panics
    ///
    /// The `new` function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }

        ThreadPool { workers, sender }
    }

    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.send(job).unwrap();
    }
}

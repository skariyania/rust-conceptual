use std::{
    sync::{mpsc, Arc, Mutex},
    thread,
};

type Job = Box<dyn FnOnce() + Send + 'static>;
type Receiver = Arc<Mutex<mpsc::Receiver<Job>>>;

/// Represents a worker in the web server.
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
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
    fn new(id: usize, receiver: Receiver) -> Worker {
        let thread = thread::spawn(move || loop {
            let message = receiver.lock().unwrap().recv();
            match message {
                Ok(job) => {
                    println!("Worker {} got a job; executing.", id);
                    job();
                }
                Err(_) => {
                    println!("Worker {} disconnected; shutting down.", id);

                    break;
                }
            }
        });

        Worker {
            id,
            thread: Some(thread),
        }
    }
}

/// A thread pool that manages a collection of worker threads.
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
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
    /// # Panics
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

        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    /// Executes a closure in the thread pool.
    ///
    /// # Arguments
    ///
    /// * `f` - The closure to be executed in parallel.
    ///
    /// # Panics
    ///
    /// The `execute` function will panic if the sender channel is closed.
    pub fn execute<F>(&self, f: F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

/// Implements the `Drop` trait for the `ThreadPool` struct.
///
/// When an instance of `ThreadPool` goes out of scope, this `drop` method is automatically called.
/// It sends a terminate message to all workers and shuts down all workers by joining their threads.
impl Drop for ThreadPool {
    /// Drops the `WebServer` instance, sending terminate messages to all workers and shutting them down.
    fn drop(&mut self) {
        println!("Sending terminate message to all workers.");

        drop(self.sender.take());

        println!("Shutting down all workers.");

        for worker in &mut self.workers {
            println!("Shutting down worker {}.", worker.id);

            if let Some(thread) = worker.thread.take() {
                thread.join().unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thread_pool_new() {
        let pool = ThreadPool::new(4);
        assert_eq!(pool.workers.len(), 4);
        assert!(pool.sender.is_some());
    }

    #[test]
    #[should_panic]
    fn test_thread_pool_new_zero_size() {
        ThreadPool::new(0);
    }

    #[test]
    fn test_thread_pool_execute() {
        let pool = ThreadPool::new(4);
        let counter = Arc::new(Mutex::new(0));

        for _ in 0..10 {
            let counter = Arc::clone(&counter);
            pool.execute(move || {
                let mut num = counter.lock().unwrap();
                *num += 1;
            });
        }

        // Wait for all tasks to complete
        std::thread::sleep(std::time::Duration::from_secs(1));

        let counter = counter.lock().unwrap();
        assert_eq!(*counter, 10);
    }
}

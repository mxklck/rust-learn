use std::sync::{Arc, Mutex, mpsc};
use std::thread;

// implementing a trait object here
type Job = Box<dyn FnOnce() + Send + 'static>;

struct Worker {
    id: usize,
    thread: thread::JoinHandle<()>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker {
        let thread = thread::spawn(move || {
            loop {
                // call lock to get mutex
                // Mutex ensures that only one Worker is trying to request a job at a time
                let message = receiver.lock().unwrap().recv();
                // let drops the mutex as soon as assignment is finished, hence this works.

                // gracefully shut down
                match message {
                    Ok(job) => {
                        println!("Worker {id} got a job; executing.");
                        job();
                    }
                    Err(_) => {
                        println!("Worker {id} disconnected; shutting down.");
                        break;
                    }
                }
            }
        });
        Worker { id, thread }
    }
}

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>, // use Option so we can take the value
}

impl ThreadPool {
    /// Create a new ThreadPool
    ///
    /// The size is the number of threads in the pool.
    ///
    /// # Panics
    ///
    /// The 'new' function will panic if the size is zero.
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let (sender, receiver) = mpsc::channel();
        // I'm doing this but I'm not sure how it works
        let receiver = Arc::new(Mutex::new(receiver));

        // workers share ownership of the receiver in an arc (mutex)
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            workers,
            sender: Some(sender),
        }
    }

    pub fn execute<F>(&self, f: F)
    where
        // we need 'static life time because we don't know
        // how long the thread will take to execute
        // () after FnOnce represents a closure that takes no parameters and returns the unit type ()
        F: FnOnce() + Send + 'static,
    {
        // f is a closure
        let job = Box::new(f);
        // not sure I fully understand this right now.
        self.sender.as_ref().unwrap().send(job).unwrap();
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take()); // explicitly drop the sender
        // dropping the sender closes the channel

        // drain moves the vector to an iterator
        for worker in self.workers.drain(..) {
            println!("Shutting down worker {}", worker.id);
            worker.thread.join().unwrap();
        }
    }
}

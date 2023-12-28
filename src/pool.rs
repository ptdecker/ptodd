//! Basic Thread Pool Implementation
use log::debug;
use std::{
    fmt,
    sync::{mpsc, Arc, Mutex},
    thread,
};

#[derive(Debug)]
pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

type Job = Box<dyn FnOnce() + Send + 'static>;

impl ThreadPool {
    /// Creates a new thread pool
    ///
    /// Creates a thread pool of size 'size'.
    ///
    /// # Panics
    ///
    /// `new` will panic if the requested size of the pool is not greater than 1
    ///
    pub fn build(size: usize) -> Result<ThreadPool, Error> {
        if size == 0 {
            return Err(Error::Channel(
                "Cannot create a zero sized thread pool".to_string(),
            ));
        }
        let (sender, receiver) = mpsc::channel();
        let receiver = Arc::new(Mutex::new(receiver));
        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver))?);
        }
        Ok(ThreadPool {
            workers,
            sender: Some(sender),
        })
    }

    pub fn execute<F>(&self, f: F) -> Result<(), Error>
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender
            .as_ref()
            .ok_or(Error::Channel(
                "expected 'sender' channel was 'None'".to_string(),
            ))?
            .send(job)?;
        Ok(())
    }
}

impl Drop for ThreadPool {
    fn drop(&mut self) {
        drop(self.sender.take());
        for worker in &mut self.workers {
            debug!("Shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
                thread.join().expect("unable to join associated thread");
            }
        }
    }
}

#[derive(Debug)]
struct Worker {
    id: usize,
    thread: Option<thread::JoinHandle<()>>,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Result<Worker, Error> {
        let thread = thread::spawn(move || loop {
            let message = receiver
                .lock()
                .expect("unable to lock spawned thread")
                .recv();
            match message {
                Ok(job) => {
                    debug!("Worker {id} got a job; executing.");
                    job();
                }
                Err(_) => {
                    debug!("Worker {id} shutting down.");
                    break;
                }
            }
        });
        Ok(Worker {
            id,
            thread: Some(thread),
        })
    }
}

#[derive(Debug)]
pub enum Error {
    Channel(String),
}

impl<T> From<mpsc::SendError<T>> for Error {
    fn from(err: mpsc::SendError<T>) -> Self {
        Error::Channel(err.to_string())
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::Channel(s) => write!(f, "channel error: {}", s),
        }
    }
}

use std::sync::{Arc, Mutex, mpsc};
use std::{thread};
use crate::core::worker::Worker;
use crate::core::job::Job;

pub struct ThreadPool {
    workers: Vec<Worker>,
    sender: Option<mpsc::Sender<Job>>,
}

impl ThreadPool {

    /// create a new threadpool,
    /// if size <= 0, panic
    /// 
    pub fn new(size: usize) -> ThreadPool{
        assert!(size > 0);
        let (sender, receiver) = mpsc::channel(); 
        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = Vec::with_capacity(size);
        for id in 0..size {
            workers.push(Worker::new(id, receiver.clone()))
        }

        ThreadPool {
            workers , 
            sender: Some(sender),
        }
    }

    pub fn execute<F> (&self, f:F)
    where
        F: FnOnce() + Send + 'static,
    {
        let job = Box::new(f);
        self.sender.as_ref().unwrap().send(job).unwrap();
    }


    
}

impl Drop for ThreadPool {
    fn drop(&mut self) {

        drop(self.sender.take());
        for worker in &mut self.workers{
            println!("shutting down worker {}", worker.id);
            if let Some(thread) = worker.thread.take() {
               thread.join().unwrap();
            }
        }
    }
}

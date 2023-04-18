use std::{thread};
use std::sync::{Arc, Mutex, mpsc};
use crate::core::job::Job;

pub struct Worker {
    pub id: usize,
    pub thread: Option<thread::JoinHandle<()>>
}

impl Worker {
    pub fn new(id:usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) ->Worker {
        let thread = thread::spawn(move|| loop {
            let message = receiver.lock().unwrap().recv();
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
        });
        Worker { id:id, thread: Some(thread) }
    }
    
}
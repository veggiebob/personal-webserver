use std::sync::{Arc, mpsc, Mutex};
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use crate::info;

type Job = Box<dyn FnOnce() + Send + 'static>;

pub struct Worker {
    id: usize,
    thread: std::thread::JoinHandle<()>
}

pub struct ThreadPool {
    sender: Sender<Job>,
    workers: Vec<Worker>
}

impl ThreadPool {

    pub fn new(num_workers: usize) -> ThreadPool {

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        let mut workers = vec![];
        for id in 0..num_workers {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        ThreadPool {
            sender,
            workers
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static {
        self.sender.send(Box::new(f));
    }
}

impl Worker {

    pub fn new(id: usize, receiver: Arc<Mutex<Receiver<Job>>>) -> Worker {
        let join_handle = thread::spawn(move || {
            let receiver = receiver;
            loop {
                let receiver = Arc::clone(&receiver);
                thread::spawn(
                    move || loop {
                        if let Ok(job) = Worker::get_job(&receiver) {
                            // println!("Worker {} processing a job!", id);
                            job();
                        } // skip over bad unwraps
                    }
                ).join();
                // println!("worker {} died! restarting!", id);
            }
        });
        Worker {
            id,
            thread: join_handle
        }
    }

    fn get_job(receiver: &Arc<Mutex<Receiver<Job>>>) -> Result<Job, ()> {
        receiver.lock().map_err(|_| ())?.recv().map_err(|_| ())
    }

}
use std::thread;
use std::sync::{mpsc, Arc, Mutex};

pub struct ThreadPool {
    #[allow(dead_code)]
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    pub fn new(size: usize) -> ThreadPool {
        assert!(size > 0);

        let mut workers = Vec::with_capacity(size);

        let (sender, receiver) = mpsc::channel();

        let receiver = Arc::new(Mutex::new(receiver));

        for id in 0..size {
            workers.push(Worker::new(id, Arc::clone(&receiver)));
        }
        
        ThreadPool {
            workers,
            sender,
        }
    }

    pub fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}


struct Worker {
    #[allow(dead_code)]
    id: usize,
}

impl Worker {
    fn new(id: usize, receiver: Arc<Mutex<mpsc::Receiver<Job>>>) -> Worker{
        thread::spawn(move || {
            loop {
                let func = receiver.lock().unwrap().recv();
                func.unwrap()();
            }
        });


        Worker {
            id
        }
    }

}


type Job = Box<dyn FnOnce() + Send + 'static>;
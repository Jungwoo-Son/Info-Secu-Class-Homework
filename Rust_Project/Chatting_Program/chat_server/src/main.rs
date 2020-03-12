use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::sync::{mpsc, Arc, Mutex};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();
    let pool = ThreadPool::new(15);
    
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut user = stream.unwrap();
            println!("{} 서버 입장", user.peer_addr().unwrap().port());
            pool.execute(move || {
                loop {
                    let mut buff = [0; 512];
                    match user.read(&mut buff) {
                        Err(_) => break,
                        Ok(_)  => {
                            println!("{} |> {}", user.peer_addr().unwrap().port() ,String::from_utf8_lossy(&buff));
                        },
                    }
                }
                println!("{} 서버 나감", user.peer_addr().unwrap().port());
            });
        }
    });







    //handle.join().unwrap();
    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
}


struct ThreadPool {
    workers: Vec<Worker>,
    sender: mpsc::Sender<Job>,
}

impl ThreadPool {
    fn new(size: usize) -> ThreadPool {
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

    fn execute<F>(&self, f: F)
        where F: FnOnce() + Send + 'static 
    {
        self.sender.send(Box::new(f)).unwrap();
    }
}

struct Worker {
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
use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;
use std::net::TcpStream;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();

    let handle = thread::spawn(move || {
        println!("{:?}", listener);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            stream.write(&[1, 10]).unwrap();
        }    
    });

    handle.join().unwrap();
    
}

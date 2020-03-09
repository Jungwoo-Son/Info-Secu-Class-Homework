use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();

    let handle = thread::spawn(move || {
        println!("{:?}", listener);
        for stream in listener.incoming() {
            let mut user = stream.unwrap();

            thread::spawn(move || {
                loop {
                    let mut buff = [0; 512];
                    match user.read(&mut buff){
                        Err(_) => break,
                        Ok(_) => {
                            println!("{}",String::from_utf8_lossy(&buff));
                        },
                    }
                }
            });
        }
    });



    //handle.join().unwrap();
    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
}

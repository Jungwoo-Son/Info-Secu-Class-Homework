use std::io::prelude::*;
use std::thread;
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();

    //사용자를 받는 스레드
    let handle = thread::spawn(move || {
        println!("{:?}", listener);
        for stream in listener.incoming() {
            let mut user = stream.unwrap();

            //사용자의 메세지를 받는 스레드
            thread::spawn(move || {
                loop {
                    let mut buff = [0; 512];
                    match user.read(&mut buff){
                        Err(_) => break,
                        Ok(_) => {
                            println!("{} |> {}", user.peer_addr().unwrap().port() ,String::from_utf8_lossy(&buff));
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

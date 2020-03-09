use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8085"){
        println!("Success");

        loop {
            let mut msg = String::new();
            std::io::stdin().read_line(&mut msg).unwrap();
            let msg = String::from(msg.trim());
            println!("|> {}", msg);
            stream.write(msg.as_bytes()).unwrap();
        }
    }
    else {
        println!("fail");
    }
}

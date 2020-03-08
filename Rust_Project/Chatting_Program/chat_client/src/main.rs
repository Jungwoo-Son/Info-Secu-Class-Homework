use std::io::prelude::*;
use std::net::TcpStream;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8085"){
        println!("Success");
        let mut buf = [0; 10];
        stream.read(&mut buf).unwrap();
        println!("{:?}", buf);
    }
    else {
        println!("fail");
    }



    
}

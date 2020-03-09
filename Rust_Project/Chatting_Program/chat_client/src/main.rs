use std::io::prelude::*;
use std::net::TcpStream;

fn input_console() -> String{
    print!("> ");
    std::io::stdout().flush().unwrap();

    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
    let msg = String::from(msg.trim());
    msg
}

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8085"){
        println!("{}",stream.local_addr().unwrap().port());
        loop {
            let msg = input_console();
            stream.write(msg.as_bytes()).unwrap();
        }
    }
    else {
        println!("서버에 연결 할 수 없습니다");
    }
}

extern crate chat_server;

use std::thread;
use std::net::TcpListener;

use chat_server::thread_pool::ThreadPool;
use chat_server::user::User;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:8085").unwrap();
    let pool = ThreadPool::new(15);
    
    thread::spawn(move || {
        for stream in listener.incoming() {
            let mut user = User::new(stream.unwrap());
            println!("{} 서버 입장", user.get_id());
            pool.execute(move || {
                loop {
                    let mut buff = [0; 512];
                    match user.read(&mut buff) {
                        Err(_) => break,
                        Ok(_)  => {
                            println!("{} |> {}", user.get_id() ,String::from_utf8_lossy(&buff));
                            user.write(String::from("너가 보냄").as_bytes()).unwrap();
                        },
                    }
                }
                println!("{} 서버 나감", user.get_id());
            });
        }
    });







    //handle.join().unwrap();
    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
}


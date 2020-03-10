extern crate rust_thread_pool;
use rust_thread_pool::ThreadPool;

use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let user = stream.unwrap();

        pool.execute(||{
            println!("afsd");
        });
    }
}

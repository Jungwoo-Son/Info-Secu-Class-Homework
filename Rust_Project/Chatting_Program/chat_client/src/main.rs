use std::io::prelude::*;
use std::net::TcpStream;

use std::thread;

fn input_console() -> String{
    std::io::stdout().flush().unwrap();

    let mut msg = String::new();
    std::io::stdin().read_line(&mut msg).unwrap();
    let msg = String::from(msg.trim());
    msg
}

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8085"){
        run_recv_chat(stream.try_clone().unwrap());
        loop {
            let msg = input_console();
            stream.write(msg.as_bytes()).unwrap();
        }
    }
    else {
        println!("서버에 연결 할 수 없습니다");
    }
}

use std::io::stdout;
use crossterm::{execute, style, terminal, cursor};

struct UiManager {
    line: u16,
}

impl UiManager {
    fn new() -> UiManager{
        UiManager::print_input_box();
        UiManager{
            line: 0,
        }
    }
    
    fn print_input_box() {
        execute!(stdout(), cursor::MoveTo(0, terminal::size().unwrap().1 - 3)).unwrap();

        let size_x = terminal::size().unwrap().0 - 1;
        print!("┌ 입력 창");
        for _ in 0..(size_x - 10) {
            print!("─");
        }
        println!("┐");

        print!("│");
        for _ in 0..(size_x - 2) {
            print!(" ");
        } println!("│");

        print!("└");
        for _ in 0..(size_x - 2) {
            print!("─");
        } print!("┘");
        std::io::stdout().flush().unwrap();
        execute!(stdout(), cursor::MoveTo(2, terminal::size().unwrap().1 - 2)).unwrap();
    }

    fn print_chat(&mut self, chat: &String) {
        let term_y = terminal::size().unwrap().1;
        execute!(stdout(), cursor::MoveTo(0, self.line)).unwrap();
        println!("{}", chat);
        if (term_y - 3) > self.line {
            self.line += 1;        }
        UiManager::print_input_box();
    }
}




fn run_recv_chat(stream: TcpStream) {
    let mut stream = stream;
    thread::spawn(move || {
        let mut ui_manager = UiManager::new();
        loop{
            let mut buffer = [0; 512];
            stream.read(&mut buffer).unwrap();
            let data = String::from(String::from_utf8_lossy(&buffer));
            //println!("{}", data);
            ui_manager.print_chat(&data);
        }
    });
}


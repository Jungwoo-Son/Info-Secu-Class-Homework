extern crate chat_client;

use std::io::prelude::*;

use std::sync::mpsc::TryRecvError;
use std::net::TcpStream;

use chat_client::net::run_recv_chat;
use chat_client::ui::UiManager;
use chat_client::console::run_input_console;

fn main() {
    if let Ok(mut stream) = TcpStream::connect("127.0.0.1:8085"){
        let mut ui_manager = UiManager::new();
        let chat_receiver = run_recv_chat(stream.try_clone().unwrap());
        let console_receiver = run_input_console();

        loop {
            match console_receiver.try_recv() {
                Ok(msg) => {stream.write(msg.as_bytes()).unwrap();},
                Err(TryRecvError::Empty) => {},
                _ => break,
            };
            
            match chat_receiver.try_recv() {
                Ok(chat_data) => ui_manager.print_chat(&chat_data),
                Err(TryRecvError::Empty) => continue,
                _ => break,
            };

<<<<<<< HEAD
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
=======

        }

    }
    else {
        println!("서버에 연결 할 수 없습니다");
>>>>>>> c00c33f95fa915d7098e8602a70841520ca89c12
    }
}
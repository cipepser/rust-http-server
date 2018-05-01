use std::net::TcpListener;
use std::thread;

fn main() {
    let lisner = TcpListener::bind("localhost:8080").unwrap();
    
    for stream in lisner.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(_) => { panic!("connection failed") }
        }
    }
}

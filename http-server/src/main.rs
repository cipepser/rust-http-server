use std::str;
use std::net::{TcpListener, TcpStream};
use std::io;
use std::io::{Write, BufRead};
use std::thread;
use std::path::PathBuf;
use std::fs::File;
use std::error::Error;

fn main() {
    let listener = TcpListener::bind("localhost:8080").unwrap();
    
    for stream in listener.incoming() {
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

fn handle_client(stream: TcpStream) {
    let mut stream = io::BufReader::new(stream);
    
    let mut first_line = String::new();
    if let Err(err) = stream.read_line(&mut first_line) {
        panic!("error during recieve a line: {}", err);
    }
    
    let mut params = first_line.split_whitespace();
    let method = params.next();
    let path = params.next();
    match (method, path) {
        (Some("GET"), Some(path)) => {
            get_operation(path, stream.get_mut());
        },
        _ => panic!("failed to parse"),
    }
}

fn get_operation(file_name: &str, stream: &mut TcpStream) {
    let path = PathBuf::from(format!("./www{}", file_name));
    let mut file = match File::open(&path) {
        Err(why) => {
            panic!(
                "couldn't open {}: {}",
                path.display(),
                Error::description(&why)
            )
        }
        Ok(file) => file,
    };
    let len =file.metadata().map(|m| m.len()).unwrap_or(0);
    
    writeln!(stream, "HTTP1.1 200 OK").unwrap();
    writeln!(stream, "Content-Type: text/html; charset=UTF-8").unwrap();
    writeln!(stream, "Content-Length: {}", len).unwrap();
    writeln!(stream).unwrap();
    
    io::copy(&mut file, stream).unwrap();
}
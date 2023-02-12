use std::time::Duration;
use std::{fs, thread};
use std::net::TcpListener;
use std::io::prelude::*;
use std::net::TcpStream;
use multi_threaded_web_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);
    
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        
        pool.execute(|| {
            handle_connection(stream);
        });
    }
    
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"sleep / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "src/hello.html")
    } else {
        ("HTTP/1.1 400 NOT FOUND", "src/404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    stream.write(contents.as_bytes()).unwrap();
    stream.flush().unwrap();
}

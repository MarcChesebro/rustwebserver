use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::fs;
use rust_webserver::ThreadPool;

fn main() {
    // set up a TCP listener on port 7878
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(6);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {

    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let index_get = b"GET / HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(index_get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();
    let headers = format!("Content-Length: {}", contents.len());

    let response = format!("{}\r\n{}\r\n\r\n{}", status_line, headers, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

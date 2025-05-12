use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK";

fn handle_connection(i: &usize, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("[{}] => [{}]", i + 1, http_request[0]);

    let content = fs::read_to_string("index.html").unwrap();
    let response = format!("\
        {RESPONSE_OK}\r\n\
        Content-Length: {}\r\n\r\n\
        {content}",
        content.len()
    );

    println!("{response}");

    stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for (i, stream) in listener.incoming().enumerate() {
        handle_connection(&i, stream.unwrap());
    }
}

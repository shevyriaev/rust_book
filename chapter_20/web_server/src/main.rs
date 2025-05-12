use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const RESPONSE_OK:&[u8] = "HTTP/1.1 200 OK".as_bytes();

fn handle_connection(i: &usize, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("[{}] => [{}]", i + 1, http_request[0]);
    stream.write_all(RESPONSE_OK).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for (i, stream) in listener.incoming().enumerate() {
        handle_connection(&i, stream.unwrap());
    }
}

use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};

const RESPONSE_OK: &str = "HTTP/1.1 200 OK";
const RESPONSE_NOT_FOUND: &str = "HTTP/1.1 404 NOT FOUND";

fn handle_connection(i: &usize, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_header = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("[{}] => [{}]", i + 1, request_header);

    let (response_status, filename) =
        if request_header == "GET / HTTP/1.1" {
            (RESPONSE_OK, "index.html")
        } else {
            (RESPONSE_NOT_FOUND, "404.html")
        };
        println!("[{}] <= [{}]", i + 1, response_status);

        let content = fs::read_to_string(filename).unwrap();
        let response = format!(
            "\
                {response_status}\r\n\
                Content-Length: {}\r\n\r\n\
                {content}\
            ",
            content.len()
        );

        stream.write_all(response.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for (i, stream) in listener.incoming().enumerate() {
        handle_connection(&i, stream.unwrap());
    }
}

use std::io::{BufRead, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_connection(i: &usize, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    println!("===> [Request no.{}]", i + 1);
    println!("{:#?}", http_request)
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for (i, stream) in listener.incoming().enumerate() {
        handle_connection(&i, stream.unwrap());
    }
}

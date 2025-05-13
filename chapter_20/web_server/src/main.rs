use std::{fs, process, thread};
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream};
use std::time::Duration;
use web_server::{ThreadPool, REQUEST, STATUS, WEBFILES};

fn handle_connection(i: &usize, mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_header = buf_reader
        .lines()
        .next()
        .unwrap()
        .unwrap();

    println!("[{}] => [{}]", i + 1, request_header);

    let (response_status, filename) = match &request_header[..] {
        REQUEST::INDEX => (STATUS::OK, WEBFILES::INDEX),
        REQUEST::SLEEP => {
            thread::sleep(Duration::from_secs(5));
            (STATUS::OK, WEBFILES::INDEX)
        },
        REQUEST::EXIT => (STATUS::OK, WEBFILES::SHUTDOWN),
        _ => (STATUS::NOT_FOUND, WEBFILES::NOT_FOUND)
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

    if request_header == REQUEST::EXIT {
        process::exit(0)
    }
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for (i, stream) in listener.incoming().take(2).enumerate() {
        pool.execute(move || handle_connection(&i, stream.unwrap()));
    }

    println!("Shutting down.");
}

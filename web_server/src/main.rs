use std::fs;
use std::io::{BufRead, BufReader, Write};
use std::net::{TcpListener, TcpStream}; // not sure what this guy does...
use std::thread;
use std::time::Duration;
use web_server::ThreadPool;

fn main() {
    // 127.0.0.1 is my computer, :7878 is a port.
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // above is like new but we "bind" to a port

    let pool = ThreadPool::new(4);
    for stream in listener.incoming().take(2) {
        // iterating over connection attempts (spawns a new thread for each connection)
        let stream = stream.unwrap();
        pool.execute(|| handle_connection(stream));
    }
    println!("Shutting down.");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream); // adds buffering
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    // next gets first line of buf reader

    // [..] means we match the whole string? not sure tbh...
    let (status, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "hello.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "hello.html")
        }
        _ => ("HTTP/1.1 404 NOT FOUND", "404.html"),
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response = format!("{status}\r\nContent-Length: {length}\r\n\r\n{contents}");
    stream.write_all(response.as_bytes()).unwrap();
}

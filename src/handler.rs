use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

pub fn handle_client(mut stream: TcpStream) {
    let mut reader = BufReader::new(&stream);
    let mut request_line = String::new();

    // Read the request line
    if let Err(e) = reader.read_line(&mut request_line) {
        eprintln!("Failed to read request line: {}", e);
        return;
    }

    // Parse the request line
    let parts: Vec<&str> = request_line.trim().split_whitespace().collect();
    if parts.len() != 3 {
        let response = "HTTP/1.1 400 Bad Request\r\nContent-Length: 0\r\n\r\n";
        stream.write_all(response.as_bytes()).unwrap();
        return;
    }

    let method = parts[0];
    let path = parts[1];
    let version = parts[2];

    println!("Method: {}, Path: {}, Version: {}", method, path, version);

    // Respond with a valid HTTP message
    let response_body = format!("Hello! You requested: {}", path);
    let response = format!(
        "HTTP/1.1 200 OK\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n\r\n{}",
        response_body.len(),
        response_body
    );

    stream.write_all(response.as_bytes()).unwrap();
}

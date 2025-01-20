use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

pub fn parse_headers(reader: &mut BufReader<&TcpStream>) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    let mut line = String::new();

    while let Ok(bytes_read) = reader.read_line(&mut line) {
        if bytes_read == 0 || line == "\r\n" {
            break; // End of headers
        }

        if let Some((key, value)) = line.trim().split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        }

        line.clear(); // Clear the line buffer for the next header
    }

    headers
}

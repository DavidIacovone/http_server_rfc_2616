use std::collections::HashMap;
use std::io::{BufRead, BufReader};
use std::net::TcpStream;

/// Parses HTTP headers from a `BufReader`.
///
/// This function reads lines from the given `BufReader` and parses them into a `HashMap`
/// where the keys and values are the header names and values, respectively.
///
/// # Arguments
///
/// * `reader` - A mutable reference to a `BufReader` wrapping a `TcpStream`.
///
/// # Returns
///
/// A `HashMap` containing the parsed headers as key-value pairs.
///
/// # Examples
///
/// ```
/// use std::collections::HashMap;
/// use std::io::BufReader;
/// use std::net::TcpStream;
/// use crate::parse_headers;
///
/// let stream = TcpStream::connect("127.0.0.1:8080").unwrap();
/// let mut reader = BufReader::new(&stream);
/// let headers = parse_headers(&mut reader);
/// println!("{:?}", headers);
/// ```
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

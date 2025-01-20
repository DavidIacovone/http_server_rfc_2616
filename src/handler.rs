use std::io::{BufRead, BufReader, Write};
use std::net::TcpStream;

use crate::headers::parse_headers;
use crate::helpers::build_response;
use crate::url::parse_query_params;

pub fn handle_client(mut stream: TcpStream) {
    loop {
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

        if version != "HTTP/1.1" {
            let response = "HTTP/1.1 505 HTTP Version Not Supported\r\nContent-Length: 0\r\n\r\n";
            stream.write_all(response.as_bytes()).unwrap();
            return;
        }

        // Parse headers using the new function
        let headers = parse_headers(&mut reader);

        // Check for Connection: keep-alive in headers
        let keep_alive = headers
            .get("Connection")
            .map(|val| val.to_lowercase() == "keep-alive")
            .unwrap_or(false);

        println!("keep_alive: {}", keep_alive);

        // Extract query parameters from the path (if any)
        let (path, query) = if let Some(query_str) = path.split_once('?') {
            (query_str.0, Some(query_str.1))
        } else {
            (path, None)
        };

        // Parse query parameters if they exist
        let query_params = query.map(|q| parse_query_params(q)).unwrap_or_default();

        // Log parsed request
        println!("Method: {}, Path: {}, Version: {}", method, path, version);
        println!("Headers: {:?}", headers);
        println!("Query Params: {:?}", query_params);

        // Handle different HTTP methods
        let response_body = match method {
            "GET" => format!("GET request for: {}", path),
            "POST" => format!("POST request to: {}", path),
            "PUT" => format!("PUT request for: {}", path),
            "DELETE" => format!("DELETE request for: {}", path),
            _ => {
                let response = build_response("405 Method Not Allowed", "", false);
                stream.write_all(response.as_bytes()).unwrap();
                return;
            }
        };

        // Build and send the response
        let status_code = match method {
            "GET" => "200 OK",
            "POST" => "201 Created",
            "PUT" => "200 OK",
            "DELETE" => "200 OK",
            _ => "405 Method Not Allowed",
        };

        // Build and send the response
        let response = build_response(&status_code, &response_body, keep_alive);
        stream.write_all(response.as_bytes()).unwrap();

        // If client requested keep-alive, keep the connection open, otherwise close it.
        if !keep_alive {
            break; // Close connection after responding
        }
    }
}
pub fn build_response(status_code: &str, body: &str, keep_alive: bool) -> String {
    let connection_header = if keep_alive {
        "Connection: keep-alive"
    } else {
        "Connection: close"
    };

    // Debugging: log the keep_alive status and the headers being built
    println!("Building response with keep-alive: {}", keep_alive);

    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n{}\r\n\r\n{}",
        status_code,
        body.len(),
        connection_header,  // Ensure this is added
        body
    )
}
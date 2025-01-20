/// Builds an HTTP response.
///
/// This function constructs an HTTP response string with the given status code,
/// body, and connection header based on the keep-alive flag.
///
/// # Arguments
///
/// * `status_code` - The HTTP status code for the response.
/// * `body` - The body of the HTTP response.
/// * `keep_alive` - A boolean flag indicating whether to keep the connection alive.
///
/// # Returns
///
/// A `String` representing the complete HTTP response.
///
/// # Examples
///
/// ```
/// let response = build_response("200 OK", "Hello, world!", true);
/// println!("{}", response);
/// ```
pub fn build_response(status_code: &str, body: &str, keep_alive: bool) -> String {
    let connection_header = if keep_alive {
        "Connection: keep-alive"
    } else {
        "Connection: close"
    };

    format!(
        "HTTP/1.1 {}\r\nContent-Length: {}\r\nContent-Type: text/plain\r\n{}\r\n\r\n{}",
        status_code,
        body.len(),
        connection_header,
        body
    )
}
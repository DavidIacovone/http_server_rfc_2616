mod handler;
mod headers;
mod url;
mod helpers;
mod rate_limiter;

use std::net::TcpListener;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use handler::handle_client;
use rate_limiter::RateLimiter;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    let rate_limiter = Arc::new(Mutex::new(RateLimiter::new(100, Duration::new(60, 0)))); // Max 100 requests per minute

    println!("Server running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                let ip = stream.peer_addr()?.ip().to_string();
                let rate_limiter = Arc::clone(&rate_limiter); // Clone the Arc to share with the thread

                // Spawn a new thread to handle the client
                thread::spawn(move || handle_client(stream, ip, rate_limiter));
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

mod handler;
mod headers;
mod url;
mod helpers;

use std::net::TcpListener;
use std::thread;
use handler::handle_client;

fn main() -> std::io::Result<()> {
    let address = "127.0.0.1:8080";
    let listener = TcpListener::bind(address)?;
    println!("Server running on http://{}", address);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| handle_client(stream)); // Handle each connection in a new thread
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}

use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Read first 1024 bytes
    match stream.read(&mut buffer) {
        Ok(n) => println!(
            "Received {} bytes:\n{}",
            n,
            String::from_utf8_lossy(&buffer[..n])
        ),
        Err(e) => eprintln!("Error reading stream: {}", e),
    }

    let body = b"<html><body><main><h1>hello world</h1></main></body></html>";
    let content_length = body.len();
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
        content_length
    );

    match stream.write_all(headers.as_bytes()) {
        Ok(_) => println!("wrote headers"),
        Err(e) => eprintln!("error writing headers {}", e),
    }

    match stream.write_all(body) {
        Ok(_) => println!("wrote body"),
        Err(e) => eprintln!("error writing body {}", e),
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    for stream in listener.incoming() {
        handle_client(stream?);
    }

    Ok(())
}

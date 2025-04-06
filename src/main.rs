use notify::{Event, RecursiveMode, Result, Watcher};
use std::fs;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::sync::mpsc;

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; // Read first 1024 bytes

    let buff_result = stream.read(&mut buffer).expect("error reading buffer");

    parse_path(buff_result, buffer);

    let html_to_render = read_html("static/index.html");

    let body = html_to_render.as_bytes();
    let content_length = body.len();
    let headers = format!(
        "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\nContent-Length: {}\r\n\r\n",
        content_length
    );

    stream
        .write_all(headers.as_bytes())
        .expect("error writing headers");

    stream.write_all(body).expect("error writing body")
}

fn main() -> Result<()> {
    // let listener = TcpListener::bind("127.0.0.1:8080")?;
    // for stream in listener.incoming() {
    //     handle_client(stream?);
    // }

    // Ok(())
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("static"), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => println!("event: {:?}", event),
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}

fn read_html(file: &str) -> String {
    let html = fs::read_to_string(file).expect("could not read html file");
    html
}

fn parse_path(n: usize, buffer: [u8; 1024]) {
    let raw_request = String::from_utf8_lossy(&buffer[..n]);
    let request_path: Vec<&str> = raw_request.split(" ").collect();

    if let Some(path) = request_path.get(1) {
        println!("Path: {}", path)
    }
}

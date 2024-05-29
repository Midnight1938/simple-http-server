use http_server_starter_rust::status::HttpStatus;
use std::{
    //    fmt::{self, Display, Formatter},
    io::{self, BufRead, BufReader, Read, Write},
    net::{self, TcpListener, TcpStream},
};

fn connection_handler(mut stream: TcpStream) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    println!("Request: {}", String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let response = if buffer.starts_with(get) {
        format!("{}\r\n", HttpStatus::Ok.into_status_line())
    } else {
        format!("{}\r\n", HttpStatus::NotFound.into_status_line())
    };

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                connection_handler(_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
    Ok(())
}

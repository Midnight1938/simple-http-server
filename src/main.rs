use http_server_starter_rust::status::HttpStatus;
use std::{
    io::{self, Read, Write},
    net::TcpListener,
};

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                println!("accepted new connection");
                stream.write_fmt(format_args!("{}\r\n", HttpStatus::Ok.into_status_line()))?;
                stream.flush()?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        };
    }
    Ok(())
}
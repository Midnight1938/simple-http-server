use http_server_starter_rust::status::HttpStatus;
use std::{
    //    fmt::{self, Display, Formatter},
    io::{self, BufRead, Read, Write},
    net::{self, TcpListener, TcpStream},
    collections::HashMap
};


fn parse_headers(request: &str) -> HashMap<String, String> {
    let mut headers = HashMap::new();
    for line in request.lines().skip(1) {
        if let Some((key, value)) = line.split_once(": ") {
            headers.insert(key.to_string(), value.to_string());
        } else {
            break;
        }
    }
    headers
}

fn connection_handler(mut stream: TcpStream) -> io::Result<()>{
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let lines: Vec<&str> = request.split("\r\n").collect();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    let headers = parse_headers(&request);


    let response = match tokens.get(0) {
        Some(&"GET") => {
            if let Some(path) = tokens.get(1){
                match *path {
                    "/" => format!("{}\r\n", HttpStatus::Ok.into_status_line()),
                    content if content.starts_with("/echo") => {
                        let tkn = content.replacen("/echo/", "", 1);
                        format!("{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                HttpStatus::Ok.into_status_line(), tkn.len(), tkn)
                    }
                    "/user-agent" => {
                        let user_agent = headers.get("User-Agent").unwrap();
                        format!("{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                HttpStatus::Ok.into_status_line(), user_agent.len(), user_agent)
                    },
                    _ => format!("{}\r\n", HttpStatus::NotFound.into_status_line())
                }
                
            }
            else { format!("{}\r\n", HttpStatus::NotFound.into_status_line()) }
        }
        Some(_) => {
            println!("Unknown method: {}", tokens[0]);
            format!("{}\r\n", HttpStatus::MethodNotAllowed.into_status_line())
        }
        None => {
            println!("No method specified");
            format!("{}\r\n", HttpStatus::BadRequest.into_status_line())
        }
    };

    println!("Response: {}", response);
    stream.write(response.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                eprintln!("accepted new connection");
                if let Err(e) = connection_handler(_stream){
                    eprintln!("Error Handling Connection: {}", e)
                };
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        };
    }
    Ok(())
}

use http_server_starter_rust::status::HttpStatus;
use std::{
    io::{self, BufRead, Read, Write},
    net::{self, TcpListener, TcpStream},
    //sync::Arc,
    collections::HashMap, thread};
use std::fs::File;
use std::path::Path;


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

fn serve_file(path: &str) -> io::Result<Vec<u8>> {
    let file_path = Path::new(path);
    let mut file = File::open(file_path)?;

    let mut buff = Vec::new();
    file.read_to_end(&mut buff)?;
    Ok(buff)
}

fn connection_handler(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..]);
    println!("Request: {}", request);
    let lines: Vec<&str> = request.split("\r\n").collect();
    let tokens: Vec<&str> = lines[0].split(" ").collect();
    let headers = parse_headers(&request);


    let mut response = Vec::new();
    match tokens.get(0) {
        Some(&"GET") => {
            if let Some(path) = tokens.get(1) {
                match *path {
                    "/" => response.extend_from_slice(HttpStatus::Ok.into_status_line().as_bytes()),
                    "/user-agent" => {
                        let alt_user_agent = "Unknown".to_string();
                        let user_agent = headers.get("User-Agent").unwrap_or(&alt_user_agent);
                        response.extend_from_slice(
                            format!("{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    HttpStatus::Ok.into_status_line(), user_agent.len(), user_agent)
                                .as_bytes());
                    }
                    content if content.starts_with("/echo") => {
                        let data = content.replacen("/echo/", "", 1);
                        response.extend_from_slice(
                            format!("{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                    HttpStatus::Ok.into_status_line(), data.len(), data)
                                .as_bytes());
                    }
                    content if content.starts_with("/files/") => {
                        let file_path = &content.replacen("/files/", "", 1); // Extract the file path from the URL
                        match serve_file(file_path) {
                            Ok(buffer) => {
                                response.extend_from_slice(
                                    format!("{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n",
                                            HttpStatus::Ok.into_status_line(), buffer.len())
                                        .as_bytes());
                                response.extend_from_slice(&buffer)
                            }
                            Err(_) => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
                        }
                    }
                    _ => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
                }
            } else { response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes()) }
        }
        Some(_) => {
            println!("Unknown method: {}", tokens[0]);
            response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
        }
        None => {
            println!("No method specified");
            response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
        }
    };

    println!("Response: {:?}", response);
    stream.write_all(&response)?;
    stream.flush()?;
    Ok(())
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    //let listener = Arc::new(listener);

    for stream in listener.incoming() {
        //let listener = Arc::clone(&listener);
        match stream {
            Ok(_stream) => {
                eprintln!("accepted new connection");
                thread::spawn(move || {
                    if let Err(e) = connection_handler(_stream) {
                        eprintln!("Error Handling Connection: {}", e)
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        };
    }
    Ok(())
}

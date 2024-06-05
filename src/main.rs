use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{self, Read, Write},
    net::{TcpListener, TcpStream},
    path::Path,
    thread,
};

use http_server_starter_rust::status::HttpStatus;

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

fn serve_file(base_dir: &str, path: &str, protocol: char, data: Option<&[u8]>) -> io::Result<Vec<u8>> {
    let mut file_path = base_dir.to_string();
    // Remove trailing slashes from base_dir and leading slashes from path
    file_path = file_path.trim_end_matches('/').to_string();
    let path = path.trim_start_matches('/');
    file_path.push('/');
    file_path.push_str(path);
    println!("Attempting to open file: {:?}", &file_path);

    let mut buff = Vec::new();
    match protocol {
        'r' => {
            let mut file = File::open(Path::new(&file_path))?;
            file.read_to_end(&mut buff)?;
            Ok(buff)
        }
        'w' => {
            if let Some(data) = data {
                let mut file = File::create(&file_path)?;
                file.write_all(&data.iter().take_while(|&&c| c != 0).copied().collect::<Vec<u8>>())?;
                println!("Writing to {}", &file_path);
                Ok(Vec::new())
            } else {
                Err(io::Error::new(io::ErrorKind::InvalidInput, "No data provided for writing"))
            }
        }
        _ => Err(io::Error::new(io::ErrorKind::InvalidInput, "Unsupported protocol")),
    }
}


fn connection_handler(mut stream: TcpStream, base_dir: &str) -> io::Result<()> {
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
                    "/" => response.extend_from_slice(format!("{}\r\n", HttpStatus::Ok.into_status_line()).as_bytes()),
                    "/user-agent" => {
                        let alt_user_agent = "Unknown".to_string();
                        let user_agent = headers.get("User-Agent").unwrap_or(&alt_user_agent);
                        response.extend_from_slice(
                            format!(
                                "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                HttpStatus::Ok.into_status_line(),
                                user_agent.len(),
                                user_agent
                            )
                                .as_bytes(),
                        );
                    }
                    content if content.starts_with("/echo/") => {
                        let data = &content[6..];
                        response.extend_from_slice(
                            format!(
                                "{}Content-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}",
                                HttpStatus::Ok.into_status_line(),
                                data.len(),
                                data
                            )
                                .as_bytes(),
                        );
                    }
                    datum if datum.starts_with("/files/") => {
                        let file_path = &datum[6..];
                        match serve_file(base_dir, file_path, 'r', None) {
                            Ok(buffer) => {
                                response.extend_from_slice(
                                    format!(
                                        "{}Content-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                                        HttpStatus::Ok.into_status_line(),
                                        buffer.len()
                                    )
                                        .as_bytes(),
                                );
                                response.extend_from_slice(&buffer)
                            }
                            Err(_) => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes()),
                        }
                    }
                    _ => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes()),
                }
            } else {
                response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
            }
        }
        Some(&"POST") => {
            if let Some(path) = tokens.get(1) {
                match *path {
                    datum if datum.starts_with("/files/") => {
                        let file_path = &datum[6..];
                        let data = request.split("\r\n\r\n").nth(1).map(|d| d.as_bytes());
                        match serve_file(base_dir, file_path, 'w', data) {
                            Ok(_) => {
                                response.extend_from_slice(
                                    format!(
                                        "{}Content-Type: text/plain\r\nContent-Length: 0\r\n\r\n",
                                        HttpStatus::Created.into_status_line()
                                    )
                                        .as_bytes(),
                                );
                            }
                            Err(_) => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotModified.into_status_line()).as_bytes()),
                        }
                    }
                    _ => response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes()),
                }
            } else {
                response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
            }
        }
        Some(_) => {
            println!("Unknown method: {}", tokens[0]);
            response.extend_from_slice(format!("{}\r\n", HttpStatus::NotFound.into_status_line()).as_bytes())
        }
        None => {
            println!("No method specified");
            response.extend_from_slice(format!("{}\r\n", HttpStatus::ImATeapot.into_status_line()).as_bytes())
        }
    };

    println!("Response: {:?}", response);
    stream.write_all(&response)?;
    stream.flush()?;
    Ok(())
}

fn parse_args() -> String {
    let args: Vec<String> = env::args().collect();
    if let Some(index) = args.iter().position(|arg| arg == "--directory") {
        if let Some(dir) = args.get(index + 1) {
            return dir.clone();
        }
    }
    "files".to_string() // Default directory
}

fn main() -> Result<(), std::io::Error> {
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();
    let base_directory = parse_args();
    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                let base_dir = base_directory.clone();
                eprintln!("accepted new connection");
                thread::spawn(move || {
                    if let Err(e) = connection_handler(_stream, &base_dir) {
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

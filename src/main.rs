use std::{
    fs::File,
    io::{Read, Write},
    net::TcpListener,
    path::Path,
};

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() < 2 {
        println!("Usage: {} --directory /home/himu/RustroverProjects/", args[0]);
        return;
    }

    let directory = match args.get(1) {
        Some(dir) if dir == "--directory" => match args.get(2) {
            Some(dir) => dir,
            None => panic!("Failed to get directory argument"),
        },
        _ => panic!("Invalid arguments"),
    };

    // Bind the server and handle incoming connections
    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    loop {
        for stream in listener.incoming() {
            match stream {
                Ok(mut stream) => {
                    let mut buffer = [0; 1024];
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

                    let mut lines = request.lines();
                    let first_line = lines.next().unwrap();
                    let mut parts = first_line.split_whitespace();

                    let method = parts.next().unwrap();
                    let path = parts.next().unwrap();
                    let _ = parts.next(); // Skipping HTTP version

                    if method == "GET" && path.starts_with("/files/") {
                        let file_name = &path[7..]; // Extract filename from path
                        let file_path = Path::new(directory).join(file_name);

                        if let Ok(mut file) = File::open(&file_path) {
                            let mut contents = Vec::new();
                            file.read_to_end(&mut contents).unwrap();

                            let response = format!(
                                "HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n",
                                contents.len()
                            );

                            stream.write_all(response.as_bytes()).unwrap();
                            stream.write_all(&contents).unwrap();
                        } else {
                            let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                            stream.write_all(response.as_bytes()).unwrap();
                        }
                    } else {
                        let response = "HTTP/1.1 404 Not Found\r\n\r\n";
                        stream.write_all(response.as_bytes()).unwrap();
                    }
                }
                Err(e) => {
                    println!("error: {}", e);
                }
            }
        }
    }
}

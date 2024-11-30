use std::{
    collections::HashMap,
    env,
    fs::File,
    io::{BufRead, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};
use threadpool::ThreadPool;

#[derive(Debug)]
struct Request {
    method: String,
    path: String,
    headers: HashMap<String, String>,
    data: Option<String>,
}

impl Request {
    fn new(reader: &mut BufReader<&TcpStream>) -> Self {
        let request_vec: Vec<String> = reader
            .lines()
            .map(|result| result.unwrap())
            .take_while(|line| !line.is_empty())
            .collect();

        let request_line = request_vec[0].split(" ").collect::<Vec<&str>>();
        let mut header_map: HashMap<String, String> = HashMap::new();
        for header in request_vec[1..].to_vec() {
            if let Some((key, value)) = header.split_once(":") {
                header_map.insert(key.trim().to_string(), value.trim().to_string());
            }
        }

        let method = request_line[0].to_string();
        let mut data_opt: Option<String> = None;
        if method == "POST" {
            let buffer_size: usize = header_map
                .get("Content-Length")
                .expect("POST requests require Content-Length header")
                .parse()
                .expect("Content-Length must be parseable to usize");

            let mut data_buffer = vec![0; buffer_size];
            let _ = reader
                .read(&mut data_buffer)
                .expect("Data must be valid UTF-8 :(");
            let data_string = String::from_utf8_lossy(&data_buffer).to_string();
            data_opt = Some(data_string);
        }

        Request {
            method: method,
            path: request_line[1].to_string(),
            headers: header_map,
            data: data_opt,
        }
    }
}

#[derive(Debug)]
struct Response {
    code: String,
    content: Option<String>,
    content_type: Option<String>,
}

impl Response {
    fn not_found() -> Self {
        Self {
            code: "404 Not Found".to_string(),
            content: None,
            content_type: None,
        }
    }

    fn bad_request() -> Self {
        Self {
            code: "400 Bad Request".to_string(),
            content: None,
            content_type: None,
        }
    }

    fn as_string(self) -> String {
        let mut response_string = format!("HTTP/1.1 {}\r\n", self.code);

        if let Some(content) = self.content {
            let content_length = content.chars().count();
            let content_type = self
                .content_type
                .expect("Content-Type header required if response has content");
            response_string = format!(
                // add response headers
                "{}Content-Type: {}\r\nContent-Length: {}\r\n\r\n{}",
                response_string, content_type, content_length, content
            );
        } else {
            // if no other headers to add, add here
            response_string = format!("{}\r\n", response_string);
        }
        response_string
    }
}

fn fetch_directory_arg() -> String {
    // Checks for directory arg in function if required by request
    let args: Vec<String> = env::args().collect();
    let dir_idx = args
        .iter()
        .position(|arg| arg == "--directory")
        .expect("Server requires 'directory' argument");
    args.get(dir_idx + 1)
        .expect("Server requires 'directory' argument")
        .clone()
}

fn generate_response(request: Request) -> Response {
    let ok_str = "200 OK".to_string();

    if request.method == "GET" {
        match request.path.as_str() {
            // GET from /
            "/" => Response {
                code: ok_str,
                content: None,
                content_type: None,
            },

            // GET from /echo/something_to_echo
            path if path.starts_with("/echo/") => {
                let content = &path[6..];
                Response {
                    code: ok_str,
                    content: Some(content.to_string()),
                    content_type: Some("text/plain".to_string()),
                }
            }

            // GET from /files/file_name
            path if path.starts_with("/files/") => {
                let dir_path = fetch_directory_arg();
                let file_name = &path[7..];
                let file_path = PathBuf::from(dir_path).join(file_name);

                if let Ok(mut file) = File::open(&file_path) {
                    let mut contents = String::new();
                    if let Ok(_) = file.read_to_string(&mut contents) {
                        Response {
                            code: ok_str,
                            content: Some(contents),
                            content_type: Some("application/octet-stream".to_string()),
                        }
                    } else {
                        Response::not_found()
                    }
                } else {
                    Response::not_found()
                }
            }

            // GET from /user-agent
            "/user-agent" => match request.headers.get("User-Agent") {
                Some(user_agent) => Response {
                    code: ok_str,
                    content: Some(user_agent.clone()),
                    content_type: Some("text/plain".to_string()),
                },
                None => Response::bad_request(),
            },

            // GET from anything else
            _ => Response::not_found(),
        }
    } else if request.method == "POST" {
        match request.path.as_str() {
            // POST to /files/file_name
            path if path.starts_with("/files/") => {
                let dir_path = fetch_directory_arg();
                let file_name = &path[7..];
                let file_path = PathBuf::from(dir_path).join(file_name);
                let file_data = request.data.expect("POST request has no data field");

                let mut file = File::create(&file_path)
                    .expect(&format!("Could not create file {}", file_path.display()));
                file.write_all(file_data.as_bytes())
                    .expect(&format!("Could not write to file {}", file_path.display()));

                Response {
                    code: "201 Created".to_string(),
                    content: None,
                    content_type: None,
                }
            }

            // POST to anything else
            _ => Response::not_found(),
        }
    } else {
        Response::bad_request()
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Buffer stream and put in Request struct
    let mut reader = BufReader::new(&stream);
    let request = Request::new(&mut reader);
    println!("{:?}", request);

    // Process and send response
    let response = generate_response(request);
    println!("OUT: {:?}", response);
    if let Err(e) = stream.write_all(response.as_string().as_bytes()) {
        eprintln!("Failed to write to stream: {}", e);
    }
}

fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:4221")?;

    // using a pool rather than spawning new threads on each connection
    let thread_pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread_pool.execute(move || {
                    handle_connection(stream);
                });
            }
            Err(e) => {
                println!("Incoming stream error: {}", e);
            }
        }
    }
    Ok(())
}

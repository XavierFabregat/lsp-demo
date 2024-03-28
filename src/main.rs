use std::io::{self, BufRead, BufReader, Read};

mod logger;
mod lsp;
mod rpc;

fn main() {
    if let Err(e) = logger::get_logger("/Users/xavi/Personal/LSP/rust-lsp/server.log") {
        panic!("Failed to initialize logger: {}", e)
    }

    log::info!("Starting server...");

    let stdin = io::stdin();
    let mut reader = BufReader::new(stdin);

    let mut header = Vec::new();

    loop {
        header.clear();
        let header_str = match reader.read_until(b'\n', &mut header) {
            Ok(_) => {
                let header_str = String::from_utf8_lossy(&header);
                log::info!("Recieved header: {}", header_str);
                header_str
            }
            Err(e) => {
                log::error!("Failed to read header: {}", e);
                panic!("Failed to read header: {}", e);
            }
        };

        if header.is_empty() {
            break;
        }

        let content_length = header
            .split(|&x| x == b':')
            .nth(1)
            .and_then(|s| std::str::from_utf8(s).ok())
            .and_then(|s| s.trim().parse::<usize>().ok())
            .expect("Failed to parse Content-Length header");

        log::info!("Content-Length: {}", content_length);

        let mut skip_bytes = vec![0; 2]; // Skip the \r\n after the header
        match reader.read_exact(&mut skip_bytes) {
            Ok(_) => {}
            Err(e) => {
                log::error!("Failed to skip bytes: {}", e);
                panic!("Failed to skip bytes: {}", e);
            }
        }
        let mut content = vec![0; content_length];
        let content_str = match reader.read_exact(&mut content) {
            Ok(_) => {
                let message = String::from_utf8_lossy(&content);
                log::info!("Received message: {}", message);
                message
            }
            Err(e) => {
                log::error!("Failed to read message: {}", e);
                panic!("Failed to read message: {}", e);
            }
        };

        let request = join_request(&header_str, &content_str);
        handle_request(&request);
    }
}

fn join_request(header: &str, content: &str) -> String {
    format!("{}\r\n{}", header, content)
}

fn handle_request(msg: &str) {
    log::info!("We get here!");
    let contents = match rpc::decode_message(msg.as_bytes()) {
        Ok(v) => v,
        Err(e) => {
            log::error!("Error while decoding message: {}", e);
            panic!()
        }
    };

    match &contents.data {
        Some(d) => match d {
            serde_json::Value::Object(map) => {
                log::info!(
                    "Received message ==> method: {}, data: {:?}",
                    contents.method,
                    map
                )
            }
            serde_json::Value::Null => (),
            _ => (),
        },
        None => (),
    };

    log::info!("We are here!");
    match contents.method.as_str() {
        "initialize" => {
            log::info!("Initializing server...");
            let initialize_request = lsp::initialize::InitializeRequest::from(contents);
            match &initialize_request.params.clientInfo {
                Some(info) => match &info.version {
                    Some(version) => {
                        log::info!("Client connected: {}, {}", info.name, version)
                    }
                    None => {
                        log::info!(
                            "Client connected: {}, {}",
                            info.name,
                            String::from("No Version")
                        )
                    }
                },
                None => {
                    log::info!("Client connected with no info given.")
                }
            }
        }
        _ => {
            log::warn!("Unimplemented method: {}", contents.method);
        }
    }
}

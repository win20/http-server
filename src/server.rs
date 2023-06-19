use crate::http::ParseError;
use crate::http::{Request, Response, StatusCode};
use crate::thread_pool::ThreadPool;
use std::convert::TryFrom;
use std::net::TcpStream;
use std::{io::Read, net::TcpListener};

pub trait Handler: Send + 'static {
    fn handle_request(&mut self, request: &Request) -> Response;

    fn handle_bad_request(&mut self, e: &ParseError) -> Response {
        println!("Failed to parse request: {}", e);
        Response::new(StatusCode::BadRequest, None)
    }
}

pub struct Server {
    addr: String,
}

impl Server {
    pub fn new(addr: String) -> Self {
        Self { addr }
    }

    pub fn run(self, handler: impl Handler + Clone) {
        println!("Listening on {}", self.addr);
        let listener = TcpListener::bind(&self.addr).unwrap();
        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            let mut stream = stream.unwrap();
            let mut buffer = [0; 1024];
            let mut handler = handler.clone();

            pool.execute(move || {
                stream.read(&mut buffer).unwrap();
                println!("{}", String::from_utf8_lossy(&buffer));

                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };

                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            });
        }
    }

    fn handle_connection(mut stream: TcpStream, mut handler: impl Handler) {
        let mut buffer = [0; 1024];

        match stream.read(&mut buffer) {
            Ok(_) => {
                println!("{}", String::from_utf8_lossy(&buffer));

                let response = match Request::try_from(&buffer[..]) {
                    Ok(request) => handler.handle_request(&request),
                    Err(e) => handler.handle_bad_request(&e),
                };

                if let Err(e) = response.send(&mut stream) {
                    println!("Failed to send response: {}", e);
                }
            }
            Err(e) => println!("Failed to read from connection: {}", e),
        }
    }
}

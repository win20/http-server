use crate::route;

use super::http::{Method, Request, Response, StatusCode};
use super::server::Handler;
use std::time::Duration;
use std::{fs, thread};

#[derive(Clone)]
pub struct WebsiteHandler {
    public_path: String,
}

impl WebsiteHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }

    fn read_file(&self, file_path: &str) -> Option<String> {
        let path = format!("{}/{}", self.public_path, file_path);

        match fs::canonicalize(path) {
            Ok(path) => {
                if path.starts_with(&self.public_path) {
                    fs::read_to_string(path).ok()
                } else {
                    println!("Directory traversal attack attempted: {}", file_path);
                    None
                }
            }
            Err(_) => None,
        }
    }
}

impl Handler for WebsiteHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        let routes = route::routes();

        match request.method() {
            Method::GET => {
                let mut response = Response::new(StatusCode::NotFound, None);
                for route in routes {
                    if request.path() == route.path() {
                        response = Response::new(StatusCode::Ok, self.read_file(route.file()))
                    } else if request.path() == "/sleep" {
                        thread::sleep(Duration::from_secs(5));
                        response = Response::new(StatusCode::Ok, self.read_file(route.file()))
                    }
                }
                response
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}

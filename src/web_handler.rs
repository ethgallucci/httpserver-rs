use super::http::{ Request, Response, StatusCode, Method };
use super::server::Handler;
pub struct WebHandler {
    public_path: String
}

impl WebHandler {
    pub fn new(public_path: String) -> Self {
        Self { public_path }
    }
}

impl Handler for WebHandler {
    fn handle_request(&mut self, request: &Request) -> Response {
        match request.method() {
            Method::GET => match request.path() {
                "/" => Response::new(StatusCode::Ok, Some("<h1>Welcome</h1>".to_string())),
                "/hello" => Response::new(StatusCode::Ok, Some("<h1>hello</h1>".to_string())),
                _ => Response::new(StatusCode::NotFound, None),
            }
            _ => Response::new(StatusCode::NotFound, None),
        }
    }
}
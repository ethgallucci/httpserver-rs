#![allow(dead_code)]

use server::Server;
use std::env;
use web_handler::WebHandler;

mod server;
mod http;
mod web_handler;

fn main() {
    let public_path = env::var("PUBLIC_PATH").unwrap();
    let server = Server::new("127.0.0.1:8080".to_string());
    server.run(WebHandler::new(public_path));
}

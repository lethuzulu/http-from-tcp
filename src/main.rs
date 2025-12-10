use crate::{request::request_from_reader, server::Server};
use std::net::TcpListener;
use ctrlc;
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
mod headers;
mod request;
mod server;
mod response;

const PORT: u16 = 42069;

fn main() {
    let server = Server::serve(PORT).unwrap();

    let running = Arc::new(AtomicBool::new(true));

    let running_clone = Arc::clone(&running);
    ctrlc::set_handler(move || {
        running_clone.store(false, Ordering::SeqCst);
    }).expect("Error setting Ctrl-C handler");

    while running.load(Ordering::SeqCst) {
        std::thread::sleep(std::time::Duration::from_millis(100));
    }

    println!("Shutting down server...");
    server.close();
    println!("Server stopped");

}

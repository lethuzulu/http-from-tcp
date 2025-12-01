use std::net::TcpListener;
use crate::request::request_from_reader;
mod request;
mod headers;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:42069").unwrap();

    loop {
        let (tcp_stream, _) = tcp_listener.accept().unwrap();
        println!("Connection Established.");

        let request = request_from_reader(tcp_stream).unwrap();
        println!("Request {:?}", request)
    }
}

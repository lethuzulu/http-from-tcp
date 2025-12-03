use crate::request::request_from_reader;
use std::net::TcpListener;
mod headers;
mod request;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:42069").unwrap();

    loop {
        let (tcp_stream, __) = tcp_listener.accept().unwrap();
        println!("Connection Established.");

        let request = request_from_reader(tcp_stream).unwrap();
        println!("Request {:?}", request)
    }
}

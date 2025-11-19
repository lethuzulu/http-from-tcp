use std::net::TcpListener;

use crate::tcp_listener::get_lines_channel;

mod tcp_listener;

mod request;

fn main() {
    let tcp_listener = TcpListener::bind("127.0.0.1:42069").unwrap();

    loop {
        let (tcp_stream, _) = tcp_listener.accept().unwrap();
        println!("Connection Established.");
        let receiver = get_lines_channel(tcp_stream);

        for i in receiver {
            println!("{}", i);
        }
    }
}

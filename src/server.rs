use std::io::Write;
use std::net::{TcpListener, TcpStream};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::sync::Arc;
use crate::response::{StatusCode, write_status_line, get_default_headers, write_headers};


#[derive(Debug)]
pub struct Server {
    listener: Arc<TcpListener>,
    closed: Arc<AtomicBool>
}

impl Server {
    pub fn serve(port: u16) -> Result<Self, String> {
        let listener = Arc::new(
            TcpListener::bind(format!("127.0.0.1:{}", port))
                .map_err(|e| format!("Failed to bind: {}", e))?
        );

        let closed = Arc::new(AtomicBool::new(false));

        let cloned_listener = Arc::clone(&listener);
        let closed_clone = Arc::clone(&closed);
        thread::spawn(move || {
            for stream in cloned_listener.incoming() {
                if closed_clone.load(Ordering::SeqCst) {
                    println!("Server closed, stopping listener");
                    break;
                }
                match stream {
                    Ok(s) => {
                        thread::spawn(|| {
                            if let Err(e) = handle(s) {
                                eprintln!("Error handling connection: {}", e);
                            }
                        });
                    }
                    Err(e) => {
                            // Only log if we're NOT closed
                            if !closed_clone.load(Ordering::SeqCst) {
                                eprintln!("Error accepting connection: {}", e);
                            }
                    }
                }
            }
        });
        
        Ok(Self { listener  , closed})
    }

    pub fn close(&self) {
       self.closed.store(true, Ordering::SeqCst);

    }
}

fn handle(mut stream: TcpStream) -> Result<(), std::io::Error> {

    write_status_line(&mut stream, StatusCode::OK)?;
    let headers = get_default_headers(0);

    write_headers(&mut stream, &headers)?;
    
    // 4. Flush to ensure it's sent
    stream.flush()?;
    
    Ok(())
}
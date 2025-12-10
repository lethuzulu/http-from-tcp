use std::io::{Write, Result};
use crate::headers::Headers;

pub enum StatusCode {
    OK = 200,
    BadRequest = 400,
    InternalServerError = 500,
}


pub fn write_status_line<W: Write>(writer: &mut W, status_code: StatusCode) -> Result<()>{

    let reason = match status_code {
        StatusCode::OK => "Ok", 
        StatusCode::BadRequest => "Bad Request",
        StatusCode::InternalServerError => "Internal Server Error"
    };

    write!(writer, "HTTP/1.1 {} {}\r\n", status_code as u16, reason)?;
    Ok(())
}

pub fn get_default_headers(content_length: usize) -> Headers {
    let mut headers = Headers::new();
    
    headers.map.insert(
        "content-length".to_string(), 
        content_length.to_string()
    );
    
    headers.map.insert(
        "connection".to_string(), 
        "close".to_string()
    );
    
    headers.map.insert(
        "content-type".to_string(), 
        "text/plain".to_string()
    );
    
    headers
}

pub fn write_headers<W: Write>(
    writer: &mut W, 
    headers: &Headers
) -> Result<()> {
    // Write each header as "Key: Value\r\n"
    for (key, value) in &headers.map {
        write!(writer, "{}: {}\r\n", key, value)?;
    }
    
    // Write empty line to mark end of headers
    write!(writer, "\r\n")?;
    
    Ok(())
}

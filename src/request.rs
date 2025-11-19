use std::io::Cursor;
use std::io::Read;

#[derive(Debug)]
pub struct Request {
    pub request_line: RequestLine,
}

#[derive(Debug)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}

pub fn request_from_reader<R: Read>(mut reader: R) -> Result<Request, String> {
    let mut buffer = String::new();

    let _ = reader.read_to_string(&mut buffer).map_err(|_| "Error reading request".to_string())?;

    let (request_line_str, _rest) = buffer.split_once("\r\n").ok_or("Error reading request".to_string())?;

    let request_line = parse_request_line(request_line_str).map_err(|e| e)?;

    Ok(Request { request_line })
}

pub fn parse_request_line(request_string: &str) -> Result<RequestLine, String> {

    let request_string_parts: Vec<&str>= request_string.split_ascii_whitespace().collect();

    if request_string_parts.len() != 3 {
        return Err("Invalid Request Line".to_string());
    }

    let method = request_string_parts[0];
    let target = request_string_parts[1];
    let version = request_string_parts[2];


    let is_method_valid = validate_request_method(method);
    let is_valid_target = validate_target(target);
    let is_version_one = validate_http_version(version).map_err(|e|e)?;


    if !is_method_valid && !is_valid_target {
        return Err("Invalid Request Line".to_string());
    }

    Ok(RequestLine {
        method: method.to_string(),
        request_target: target.to_string(),
        http_version: is_version_one.to_string(),
    })
}

fn validate_request_method(method: &str) -> bool {
    let is_alphabetic = method.chars().all(|c| c.is_ascii_alphabetic());
    is_alphabetic
}

fn validate_http_version(http_version: &str) -> Result<String, String> {
    let http_parts: Vec<&str> = http_version.split('/').collect();
    let http_version = http_parts.into_iter().last().unwrap();
    let is_version_one = http_version.eq("1.1");
    if !is_version_one {
        return Err("Invalid http version".to_string())
    }
    Ok(http_version.to_string())
}

fn validate_target(target: &str) -> bool {
    target.starts_with('/')
}

#[test]
fn test_good_get_request_line() {
    let input = "\
GET / HTTP/1.1\r\n\
Host: localhost:42069\r\n\
User-Agent: curl/7.81.0\r\n\
Accept: */*\r\n\
\r\n";

    let r = request_from_reader(Cursor::new(input))
        .expect("Expected no error for valid GET request");

    assert_eq!(r.request_line.method, "GET");
    assert_eq!(r.request_line.request_target, "/");
    assert_eq!(r.request_line.http_version, "1.1");
}

#[test]
fn test_good_get_request_line_with_path() {
    let input = "\
GET /coffee HTTP/1.1\r\n\
Host: localhost:42069\r\n\
User-Agent: curl/7.81.0\r\n\
Accept: */*\r\n\
\r\n";

    let r = request_from_reader(Cursor::new(input))
        .expect("Expected no error for GET /coffee");

    assert_eq!(r.request_line.method, "GET");
    assert_eq!(r.request_line.request_target, "/coffee");
    assert_eq!(r.request_line.http_version, "1.1");
}

// #[test]
// fn test_invalid_request_line_not_enough_parts() {
//     let input = "\
// /coffee HTTP/1.1\r\n\
// Host: localhost:42069\r\n\
// User-Agent: curl/7.81.0\r\n\
// Accept: */*\r\n\
// \r\n";

//     let err = request_from_reader(Cursor::new(input))
//         .expect_err("Expected an error for invalid request line");

//     // Optionally assert type or error message:
//     // assert!(matches!(err, RequestError::InvalidRequestLine));
// }

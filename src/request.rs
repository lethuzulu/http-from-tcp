use std::io::Cursor;
use std::io::Read;
use std::io::Error as IoError;

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

pub fn request_from_reader<R: Read>(mut reader: R) -> Result<Request, RequestError> {
    let mut buffer = String::new();

    let _ = reader
        .read_to_string(&mut buffer)
        .map_err(|_| RequestError::InvalidRequest)?;

    let (request_line_str, _rest) = buffer
        .split_once("\r\n")
        .ok_or(RequestError::InvalidRequest)?;

    let request_line = parse_request_line(request_line_str).map_err(|e| e)?;

    Ok(Request { request_line })
}

pub fn parse_request_line(request_string: &str) -> Result<RequestLine, RequestError> {
    let request_string_parts: Vec<&str> = request_string.split_ascii_whitespace().collect();

    if request_string_parts.len() < 3 {
        return Err(RequestError::InvalidRequestLine);
    }

    let _method = request_string_parts.get(0).ok_or( RequestError::InvalidRequestLine)?;
    let _target = request_string_parts.get(1).ok_or( RequestError::InvalidRequestLine)?;
    let _http_version = request_string_parts.get(2).ok_or( RequestError::InvalidRequestLine)?;

    let method = validate_request_method(_method).map_err(|e| e)?;
    let request_target = validate_target(_target).map_err(|e| e)?;
    let http_version = validate_http_version(_http_version).map_err(|e| e)?;

    Ok(RequestLine {
        method,
        request_target,
        http_version
    })
}

fn validate_request_method(method: &str) -> Result<String, RequestError> {
    let is_alphabetic = method.chars().all(|c| c.is_ascii_alphabetic());
    if !is_alphabetic {
        return Err(RequestError::InvalidRequestMethod);
    }
    Ok(method.to_string())
}

fn validate_http_version(http_version: &str) -> Result<String, RequestError> {
    let http_parts: Vec<&str> = http_version.split('/').collect();
    let http_version = http_parts.into_iter().last().unwrap();
    let is_version_one = http_version.eq("1.1");
    if !is_version_one {
        return Err(RequestError::InvalidRequestHttpVersion);
    }
    Ok(http_version.to_string())
}

fn validate_target(target: &str) -> Result<String, RequestError> {
    if !target.starts_with('/') {
        return Err(RequestError::InvalidRequestTarget);
    }
    Ok(target.to_string())
}

#[test]
fn test_good_get_request_line() {
    let input = "\
GET / HTTP/1.1\r\n\
Host: localhost:42069\r\n\
User-Agent: curl/7.81.0\r\n\
Accept: */*\r\n\
\r\n";

    let r =
        request_from_reader(Cursor::new(input)).expect("Expected no error for valid GET request");

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

    let r = request_from_reader(Cursor::new(input)).expect("Expected no error for GET /coffee");

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
//         .unwrap_err();

//     // Optionally assert type or error message:
//     assert!(matches!(err, RequestError::InvalidRequestLine));
// }


#[derive(Debug)]
pub enum RequestError {
    InvalidRequest,
    InvalidRequestLine,
    InvalidRequestMethod,
    InvalidRequestTarget,
    InvalidRequestHttpVersion,
}

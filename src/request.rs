
use std::io::Cursor;
use std::io::Read;

pub struct Request {
    pub request_line: RequestLine
}


pub struct RequestLine {
    pub http_version: String,
    pub request_target: String,
    pub method: String
}



pub fn request_from_reader<R: Read>(mut reader: R)  {
    let mut request_string = String::new();

    let _d = reader.read_to_string(&mut request_string).unwrap();

    // Check here if the string is ascii, if it's not return an error

}

pub fn parse_request_line(request_string: &str) -> Result<(), String> {

    let line_break_position = request_string.find('\n').unwrap();

    let request_line_str = &request_string[0..line_break_position - 2]; // -2 to account for \r\n


    let parts: Vec<&str> = request_string.split("\r\n").collect();

    if parts.len() != 3 { return Err("invalid request line".to_string()) }

    unimplemented!()
}

fn validate_request_method(method: &str) -> bool {
    let is_alphabetic = method.chars().all(|c| c.is_ascii_alphabetic());
    is_alphabetic
}

fn validate_http_version(http_version: &str) -> bool {
    let http_parts: Vec<&str> = http_version.split('/').collect();
    let http_version = http_parts.into_iter().last().unwrap();
    let is_version_one = http_version.eq("1.1");
    is_version_one
}









































// #[test]
// fn test_good_get_request_line() {
//     let input = "\
// GET / HTTP/1.1\r\n\
// Host: localhost:42069\r\n\
// User-Agent: curl/7.81.0\r\n\
// Accept: */*\r\n\
// \r\n";

//     let r = request_from_reader(Cursor::new(input))
//         .expect("Expected no error for valid GET request");

//     assert_eq!(r.request_line.method, "GET");
//     assert_eq!(r.request_line.request_target, "/");
//     assert_eq!(r.request_line.http_version, "1.1");
// }

// #[test]
// fn test_good_get_request_line_with_path() {
//     let input = "\
// GET /coffee HTTP/1.1\r\n\
// Host: localhost:42069\r\n\
// User-Agent: curl/7.81.0\r\n\
// Accept: */*\r\n\
// \r\n";

//     let r = request_from_reader(Cursor::new(input))
//         .expect("Expected no error for GET /coffee");

//     assert_eq!(r.request_line.method, "GET");
//     assert_eq!(r.request_line.request_target, "/coffee");
//     assert_eq!(r.request_line.http_version, "1.1");
// }

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

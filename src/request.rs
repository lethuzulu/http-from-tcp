
use std::io::Error as IoError;
use std::io::Read;
use std::str::from_utf8;

const BUFFER_SIZE: usize = 8;

#[derive(Debug)]
pub enum ParseState {
    Initialized,
    Done,
}

#[derive(Debug)]
pub struct Request {
    pub state: ParseState,
    pub request_line: Option<RequestLine>,
}

impl Request {
    pub fn new() -> Self {
        Self {
            state: ParseState::Initialized,
            request_line: None,
        }
    }

    pub fn parse(&mut self, data: &[u8]) -> Result<usize, RequestError> {
        match self.state {
            ParseState::Initialized => {
                let (request_line, num_bytes) = parse_request_line(data)?;
                match request_line {
                    Some(r) => {
                        self.state = ParseState::Done;
                        self.request_line = Some(r);
                        return Ok(num_bytes);
                    }
                    None => return Ok(num_bytes),
                }
            }
            ParseState::Done => return Err(RequestError::DoneState),
        }
    }
}

pub fn request_from_reader<R: Read>(mut reader: R) -> Result<Request, RequestError> {
    let mut request = Request::new();

    let mut buffer = [0u8; BUFFER_SIZE];
    let mut accumulator: Vec<u8> = Vec::new();

    loop {
        let n = match reader.read(&mut buffer) {
            Ok(n) => n,
            Err(_) => return Err(RequestError::InvalidRequest),
        };

        if n == 0 {
            match request.state {
                ParseState::Done => return Ok(request),
                ParseState::Initialized => return Err(RequestError::InvalidRequest),
            }
        }

        accumulator.extend_from_slice(&buffer[..n]);
        match request.state {
            ParseState::Initialized => {
                let consumed = request.parse(&accumulator).unwrap();
                if consumed > 0 {
                    let _ = accumulator.drain(..consumed);
                }
            }
            ParseState::Done => return Ok(request),
        }
    }
}

pub fn parse_request_line(
    request_string: &[u8],
) -> Result<(Option<RequestLine>, usize), RequestError> {
    if let Some(pos) = request_string.windows(2).position(|w| w == b"\r\n") {
        let line = from_utf8(&request_string[..pos]).unwrap();
        let request_string_parts: Vec<&str> = line.split_ascii_whitespace().collect();
        // let request_line = parse_request_line(line).map_err(|e| e)?;
        if request_string_parts.len() < 3 {
            return Err(RequestError::InvalidRequestLine);
        }
        let method = validate_request_method(request_string_parts[0]).map_err(|e| e)?;
        let request_target = validate_target(request_string_parts[1]).map_err(|e| e)?;
        let http_version = validate_http_version(request_string_parts[2]).map_err(|e| e)?;
        return Ok((
            Some(RequestLine {
                method,
                request_target,
                http_version,
            }),
            pos + 2,
        ));
    }
    Ok((None, 0))
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

#[derive(Debug)]
pub struct RequestLine {
    pub method: String,
    pub request_target: String,
    pub http_version: String,
}

#[derive(Debug)]
pub struct ChunkReader {
    pub data: Vec<u8>,
    pub num_bytes_per_read: usize,
    pub pos: usize,
}

impl Read for ChunkReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, IoError> {
        if self.pos >= self.data.len() {
            return Ok(0);
        }
        let end = (self.pos + self.num_bytes_per_read).min(self.data.len());

        let available = end - self.pos;

        let to_copy = available.min(buf.len());

        buf[..to_copy].copy_from_slice(&self.data[self.pos..self.pos + to_copy]);

        self.pos += to_copy;
        Ok(to_copy)
    }
}

#[test]
fn test_good_get_request_line() {
    let input = "\
    GET / HTTP/1.1\r\n\
    Host: localhost:42069\r\n\
    User-Agent: curl/7.81.0\r\n\
    Accept: */*\r\n\
    \r\n";

    let chunk_reader = ChunkReader {
        data: input.as_bytes().to_vec(),
        num_bytes_per_read: 3,
        pos: 0,
    };

    let r = request_from_reader(chunk_reader).expect("Expected no error for valid GET request");
    let request_line = r.request_line.unwrap();
    assert_eq!(request_line.method, "GET");
    assert_eq!(request_line.request_target, "/");
    assert_eq!(request_line.http_version, "1.1");
}

#[test]
fn test_good_get_request_line_with_path() {
    let input = "\
    GET /coffee HTTP/1.1\r\n\
    Host: localhost:42069\r\n\
    User-Agent: curl/7.81.0\r\n\
    Accept: */*\r\n\
    \r\n";

    let chunk_reader = ChunkReader {
        data: input.as_bytes().to_vec(),
        num_bytes_per_read: 4,
        pos: 0,
    };

    let r = request_from_reader(chunk_reader).expect("Expected no error for GET /coffee");
    let request_line = r.request_line.unwrap();
    assert_eq!(request_line.method, "GET");
    assert_eq!(request_line.request_target, "/coffee");
    assert_eq!(request_line.http_version, "1.1");
}

#[test]
fn test_invalid_request_line_not_enough_parts() {
    let input = "\
    /coffee HTTP/1.1\r\n\
    Host: localhost:42069\r\n\
    User-Agent: curl/7.81.0\r\n\
    Accept: */*\r\n\
    \r\n";

    let chunk_reader = ChunkReader {
        data: input.as_bytes().to_vec(),
        num_bytes_per_read: 8,
        pos: 0,
    };

    let err = request_from_reader(chunk_reader).unwrap_err();

    // Optionally assert type or error message:
    assert!(matches!(err, RequestError::InvalidRequestLine));
    
    
}

#[derive(Debug)]
pub enum RequestError {
    InvalidRequest,
    InvalidRequestLine,
    InvalidRequestMethod,
    InvalidRequestTarget,
    InvalidRequestHttpVersion,
    InvalidHeader,
    DoneState,
}

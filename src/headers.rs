use std::collections::HashMap;
use std::str::from_utf8;

use crate::request::RequestError;

pub struct Headers {
    pub map: HashMap<String, String>,
}

impl Headers {
    pub fn new() -> Self {
        Self {
            map: HashMap::new(),
        }
    }
    pub fn parse(&mut self, data: &[u8]) -> Result<(usize, bool), RequestError> {
        let (key_value, consumed) = Self::parse_header(data)?;

        match key_value {
            Some((key, value)) => {
                let _ = self.map.insert(key, value);
                return Ok((consumed, false));
            }
            None => {
                if consumed > 0 {
                    return Ok((consumed, true));
                } else {
                    return Ok((consumed, false));
                }
            }
        }
    }

    fn parse_header(data: &[u8]) -> Result<(Option<(String, String)>, usize), RequestError> {
        if let Some(pos) = data.windows(2).position(|w| w == b"\r\n") {
            if pos == 0 {
                return Ok((None, 2));
            }
            let line = from_utf8(&data[..pos]).map_err(|_| RequestError::InvalidHeader)?;
            let (key, value) = match line.split_once(':') {
                Some((key, value)) => (key, value),
                None => return Err(RequestError::InvalidHeader),
            };
            let key = Self::validate_key(key)?;
            let value = Self::validate_value(value);
            return Ok((Some((key, value)), pos + 2));
        }
        Ok((None, 0))
    }

    fn validate_key(key: &str) -> Result<String, RequestError> {
        // Check if there's trailing whitespace (space before colon)
        if key.ends_with(' ') {
            return Err(RequestError::InvalidHeader);
        }
        // Trim leading whitespace and return 
        Ok(key.trim_start().to_lowercase())
    }

    fn validate_value(value: &str) -> String {
        value.trim().to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_single_header() {
        let mut headers = Headers::new();
        let data = b"Host: localhost:42069\r\n\r\n";
        let result = headers.parse(data);
        
        assert!(result.is_ok(), "expected no error for valid header");
        let (n, done) = result.unwrap();
        
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
        assert_eq!(n, 23);
        assert!(!done);
    }

    #[test]
    fn test_valid_single_header_with_extra_whitespace() {
        let mut headers = Headers::new();
        // Extra leading and trailing whitespace in the value
        let data = b"Host:    localhost:42069    \r\n\r\n";
        let result = headers.parse(data);
        
        assert!(result.is_ok(), "expected no error with extra whitespace");
        let (n, done) = result.unwrap();
        
        // Value should be trimmed
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
        assert_eq!(n, 30);
        assert!(!done);
    }

    #[test]
    fn test_valid_single_header_with_leading_whitespace_on_key() {
        let mut headers = Headers::new();
        // Leading whitespace before key is valid
        let data = b"          Host: localhost:42069    \r\n\r\n";
        let result = headers.parse(data);
        
        assert!(result.is_ok(), "expected no error with leading whitespace on key");
        let (n, done) = result.unwrap();
        
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
        assert_eq!(n, 37);
        assert!(!done);
    }

    #[test]
    fn test_valid_2_headers_with_existing_headers() {
        let mut headers = Headers::new();
        
        // Parse first header
        let data1 = b"Host: localhost:42069\r\n";
        let result1 = headers.parse(data1);
        assert!(result1.is_ok());
        let (n1, done1) = result1.unwrap();
        assert_eq!(n1, 23);
        assert!(!done1);
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
        
        // Parse second header (headers already exist from first parse)
        let data2 = b"Content-Type: application/json\r\n";
        let result2 = headers.parse(data2);
        assert!(result2.is_ok());
        let (n2, done2) = result2.unwrap();
        assert_eq!(n2, 32);
        assert!(!done2);
        
        // Both headers should be present
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
        assert_eq!(
            headers.map.get("Content-Type").map(|s| s.as_str()),
            Some("application/json")
        );
        assert_eq!(headers.map.len(), 2);
    }

    #[test]
    fn test_valid_done() {
        let mut headers = Headers::new();
        
        // Parse a header first
        let data1 = b"Host: localhost:42069\r\n";
        let result1 = headers.parse(data1);
        assert!(result1.is_ok());
        let (n1, done1) = result1.unwrap();
        assert_eq!(n1, 23);
        assert!(!done1);
        
        // Now parse the empty line (end of headers)
        let data2 = b"\r\n";
        let result2 = headers.parse(data2);
        assert!(result2.is_ok());
        let (n2, done2) = result2.unwrap();
        assert_eq!(n2, 2); // Consumed 2 bytes (\r\n)
        assert!(done2);     // Should signal done=true
        
        // Header should still be present
        assert_eq!(
            headers.map.get("Host").map(|s| s.as_str()),
            Some("localhost:42069")
        );
    }

    #[test]
    fn test_invalid_spacing_header() {
        let mut headers = Headers::new();
        // Space before colon is invalid
        let data = b"       Host : localhost:42069       \r\n\r\n";
        let result = headers.parse(data);
        
        assert!(result.is_err(), "expected error due to invalid spacing");
        assert!(headers.map.is_empty());
    }

    #[test]
    fn test_not_enough_data() {
        let mut headers = Headers::new();
        // No CRLF present, so not enough data
        let data = b"Host: localhost:42069";
        let result = headers.parse(data);
        
        assert!(result.is_ok());
        let (n, done) = result.unwrap();
        assert_eq!(n, 0);      // No bytes consumed
        assert!(!done);        // Not done
        assert!(headers.map.is_empty()); // Nothing parsed yet
    }
}
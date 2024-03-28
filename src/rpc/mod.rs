use std::{error::Error, str};

use serde::{Deserialize, Serialize};

pub fn encode_message<T: Serialize>(msg: T) -> String {
    let body = match serde_json::to_string(&msg) {
        Ok(json_str) => json_str,
        Err(e) => panic!("Error encoding message: {}", e),
    };

    format!("Content-Length: {}\r\n\r\n{}", body.len(), body)
}

pub fn decode_message(msg: &[u8]) -> Result<BaseMessage, Box<dyn Error>> {
    let s = match str::from_utf8(msg) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };

    let mut parts = s.splitn(2, "\r\n\r\n");
    let header = match parts.next() {
        Some(v) => v,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid header",
            )))
        }
    };

    let contents = match parts.next() {
        Some(v) => v,
        None => {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Invalid contents",
            )))
        }
    };
    log::info!("it fails here");
    let base_message = match serde_json::from_str::<BaseMessage>(contents) {
        Ok(v) => v,
        Err(e) => return Err(Box::new(e)),
    };
    log::info!("Base Message: {:?}", &base_message);

    Ok(base_message)
}
#[derive(Serialize, Deserialize, Debug)]
pub struct BaseMessage {
    pub method: String,
    pub data: Option<serde_json::Value>,
    pub params: Option<serde_json::Value>,
    pub id: Option<i32>,
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn should_encode() {
        #[derive(Serialize)]
        struct TestMessage {
            method: String,
            data: String,
        }
        let expected = "Content-Length: 32\r\n\r\n{\"method\":\"test\",\"data\":\"hello\"}";
        let msg = TestMessage {
            method: "test".to_string(),
            data: "hello".to_string(),
        };
        let encoded = encode_message(msg);
        assert_eq!(encoded, expected);
    }

    #[test]
    fn should_decode() {
        let message = "Content-Length: 32\r\n\r\n{\"method\":\"test\",\"data\":\"hello\"}";
        let contents = decode_message(message.as_bytes()).unwrap();
        let expected_contents = "{\"method\":\"test\",\"data\":\"hello\"}";
        let contents_len = serde_json::to_string(&contents).unwrap().len();
        assert_eq!(contents_len, expected_contents.len());
    }
}

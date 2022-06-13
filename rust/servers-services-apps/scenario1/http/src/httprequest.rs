use std::collections::HashMap;

mod members;
mod helpers;

use self::{ members::*, helpers::* };

// Main HTTP request struct
//  - see below for specific members
// TODO: Everything is currently pub -> change that
// TODO: Improve error handling (make options, try into, etc.)
//      - Currently, Method & Version have a baked in 'Uninitialized' member
//        which could be thought of as the 'None' equivalent
#[derive(Debug)]
pub struct HttpRequest {
    pub method: Method,
    pub version: Version,
    pub resource: Resource,
    // Might add these into the member file as well
    // so that methods can be attached to them
    // (and might rename the member file)
    pub headers: HashMap<String, String>,
    pub msg_body: Option<String>,
}

impl TryFrom<String> for HttpRequest {
    type Error = ReqError; // <-- need to implement this

    fn try_from(req: String) -> Result<Self, Self::Error> {
        // TODO: make all 'process' calls return Result
        //  -> to clean up error handling
        // First, we check that the format is correct:
        if let Err(e) = check_req_format(&req) {
            return Err(e);
        }
        let mut lineIter = req.lines();

        let (method, resource, version) = process_req_line(lineIter.next().unwrap());

        let mut headers = HashMap::new();
        loop {
            match lineIter.next() {
                Some(header) if header != "" => {
                    let (key, value) = process_header_line(header);
                    headers.insert(key, value);
                }
                _ => break,
            }
        }

        // Unsure if this works (can msg_body have newlines?)
        let msg_body = match lineIter.next() {
            Some(s) => Some(String::from(s)),
            None => None,
        };

        Ok( HttpRequest {
            method,
            version,
            resource,
            headers,
            msg_body,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_read_http() {
        let s: String = String::from("GET /greeting HTTP/1.1\r\nHost:
        localhost:3000\r\nUser-Agent: curl/7.64.1\r\nAccept: */*\r\n\r\n");

        let mut headers_expected = HashMap::new();
        headers_expected.insert("Host".into(), " localhost".into());
        headers_expected.insert("Accept".into(), " */*".into());
        headers_expected.insert("User-Agent".into(), " curl/7.64.1".into());

        let req: HttpRequest = s.into();

        assert_eq!(Method::Get, req.method);
        assert_eq!(Version::V1_1, req.version);
        assert_eq!(Resource::Path("/greeting".to_string()), req.resource);
        assert_eq!(headers_expected, req.headers);
    }
}


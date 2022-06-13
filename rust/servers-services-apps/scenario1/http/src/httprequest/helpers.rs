// Helper functions for parsing HTTP requests
use super::{Method, Resource, Version};

// Parses the request line (top of request) into the proper types
pub fn process_req_line(s: &str) -> (Method, Resource, Version) {
    // This guard clause is a little messy:
    if !req_line_format(s) {
        return (
            Method::Uninitialized,
            Resource::Path(String::new()),
            Version::Uninitialized,
        );
    }

    // TODO: implement some 'builder' or other way to guarantee that
    // process_req_line is never called without check_req_format
    let mut words = s.split_whitespace();
    // Extract HTTP method
    let method = words.next().unwrap(); // This unwrap should never panic!
    // Extract resource (URI/URL)
    let resource = words.next().unwrap();
    // Extrac HTTP version
    let version = words.next().unwrap();

    (
        method.into(),
        Resource::Path(resource.to_string()),
        version.into(),
    )  
}

// Parses header lines into key-value pairs
pub fn process_header_line(s: &str) -> (String, String) {
    if !header_line_format(s) {
        return (String::new(), String::new());
    }
    // Headerline items are separated by a ':'
    let mut header_items = s.split(":");
    // These unwraps should not panic, due to checks above
    let key = String::from(header_items.next().unwrap());
    let value = String::from(header_items.next().unwrap());

    (key, value)
}

// Tool for checking if the request of properly formatted
// TODO: might not be necessary
pub fn check_req_format(req: &str) -> bool {
    true
}

// Checking req_line format
fn req_line_format(req_line: &str) -> bool {
    true
}

// Checking header line format
fn header_line_format(header_line: &str) -> bool {
    true
}

// Checking msg_body format
fn msg_body_format(msg_body: &str) -> bool {
    true
}

// TODO: add tests

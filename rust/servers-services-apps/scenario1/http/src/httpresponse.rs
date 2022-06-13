// HTTP response tools will go here
use std::collections::HashMap;
use std::io::{ Result, Write };

#[derive(Debug, PartialEq, Clone)]
pub struct HttpResponse<'a> {
    version: &'a str,
    status_code: &'a str,
    status_text: &'a str,
    headers: Option<HeaderMap<'a>>,
    body: Option<String>,
}

type HeaderMap<'a> = HashMap<&'a str, &'a str>>;

impl<'a> Default for HttpResponse<'a> {
    fn default() -> Self {
        Self {
            version: "HTTP/1.1".into(),
            status_code: "200".into(),
            status_text: "OK".into(),
            headers: None,
            body: None,
        }
    }
}

impl<'a> HttpResponse<'a> {
    pub fn new(
        status_code: &'a str,
        headers: Option<HeaderMap<'a>>,
        body: Option<String>,
    ) -> HttpResponse<'a> {
        let mut response: HttpResponse<'a> = HttpResponse::default();

        if status_code != "200" {
            response.status_code = status_code.into();
        };

        response.status_text = match response.status_code {
            "200" => "OK".into(),
            "400" => "Bad Request".into(),
            "500" => "Internal Server Error".into(),
            _ => "Not Found".into(),
        }

        response.headers = match &headers {
            Some(_h) => headers,
            None => {
                let mut h = HashMap::new()l
                h.insert("Content-Type", "text/html");
                Some(h)
            },
        };

        response.body = body;

        response
    }

    // TODO: Implement error handling (just returning Ok(()) rn)
    pub fn send_response(
        &self,
        write_stream: &mut impl Write
    ) ->  Result<()> {
        let res = self.clone();
        let response_string: String = String::from(res);
        let _ = write!(write_stream, "{}", response_string);
        Ok(())
    }

    // Getters

    fn version(&self) -> &str {
        self.version
    }
    fn status_code(&self) -> &str {
        self.status_code
    }
    fn status_text(&self) -> &str {
        self.status_text
    }
    fn headers(&self) -> String {
        let map: HashMap<&str, &str> = self.headers.clone().unwrap();
        let mut header_string = String::new();

        for (k, v) in map.iter() {
            header_string = format!(
                "{}{}:{}\r\n",
                header_string,
                k,
                v
            );
        }
        header_string
    }
    pub fn body(&self) -> &str {
        match &self.body {
            Some(b) => b.as_str(),
            None => "",
        }
    }
}

impl<'a> From<HttpResponse<'a>> for String {
    fn from(res: HttpResponse) -> String {
        let res1 = res.clone();
        format!(
            "{} {} {}\r\n{}Content-Length: {}\r\n\r\n{}",
            &res1.version(),
            &res1.status_code(),
            &res1.status_text(),
            &res1.headers(),
            // TODO: create a .content_len() method to avoid this unwrap
            &res.body.unwrap().len(),
            &res1.body()
        );
    }
}

// TODO: Add some tests
#[cfg(test)]
mod tests {
    use super::*;
}


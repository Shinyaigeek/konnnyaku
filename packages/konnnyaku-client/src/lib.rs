use konnnyaku_common::request::{Request, RequestMethod};
use konnnyaku_common::response::Response;
use konnnyaku_common::stream::ApplicationStream;
use konnnyaku_common::tls::TlsConnector;
use konnnyaku_common::url::Url;

pub struct Client {
    tls_connector: TlsConnector,
}

impl Client {
    pub fn build() -> Self {
        return Self {
            tls_connector: TlsConnector::new(),
        };
    }

    pub fn get(url: String) -> Response {
        let url = Url::new(&Self::validate_url(url));
        let request = Request::build(url.pathname.clone(), RequestMethod::GET, url.host.clone());
        let request = request.print();
        let client = Client::build();
        let mut stream = client.connect(url);
        let request = request.as_bytes();
        stream.write(request).unwrap();
        stream.write(&[0]).unwrap();
        let response = Response::parse_stream_to_response(&mut stream);
        response
    }

    fn connect(&self, url: Url) -> ApplicationStream {
        ApplicationStream::new(&url, &self.tls_connector)
    }

    fn validate_url(url: String) -> String {
        if url.ends_with("/") {
            url
        } else {
            let mut url = url.clone();
            url.push('/');
            url
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let response = Client::get("http://example.com/".to_string());
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn it_works_with_url_not_ending_with_slash() {
        let response = Client::get("http://example.com".to_string());
        assert_eq!(response.status_code, 200);
    }

    #[test]
    fn it_works_with_https_request() {
        let response = Client::get("https://example.com".to_string());
        assert_eq!(response.status_code, 200);
    }
}

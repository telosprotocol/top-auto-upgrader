use hyper::{Body, Client, Method, Request, StatusCode};
use hyper_tls::HttpsConnector;

use crate::error::AuError;

pub struct VersionHandler {
    uri: String,
}

impl VersionHandler {
    // fn new

    async fn get_release_info_json(&self) -> Result<json::JsonValue, AuError> {
        let req = Request::builder()
            .method(Method::GET)
            .uri(&self.uri)
            .header("User-Agent", "hyper/0.14")
            .header("Accept", "application/vnd.github+json")
            .body(Body::empty())?;
        let https = HttpsConnector::new();
        let resp = Client::builder()
            .build::<_, hyper::Body>(https)
            .request(req)
            .await?;
        if let StatusCode::OK = resp.status() {
            let body_content = hyper::body::to_bytes(resp.into_body()).await?;
            let content = std::str::from_utf8(body_content.as_ref()).unwrap_or("");
            // println!("{}", content);
            return Ok(json::parse(content)?);
        }
        Err(AuError::HttpError(String::from("request error")))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    async fn do_get_release_info() -> json::JsonValue {
        let h = VersionHandler {
            uri: String::from(
                "https://api.github.com/repos/telosprotocol/TOP-Chain/releases/latest",
            ),
        };
        h.get_release_info_json().await.unwrap()
    }

    #[test]
    fn test_get_release_info() {
        let r = tokio_test::block_on(do_get_release_info());
        println!("r: {}", r);
    }
}

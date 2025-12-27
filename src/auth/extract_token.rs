use actix_web::HttpRequest;

pub fn extract_token(req: &HttpRequest) -> Option<&str> {
    req.query_string()
        .split('&')
        .find_map(|kv| kv.strip_prefix("token="))
}
#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::test::TestRequest;

    #[test]
    fn extracts_token_from_query() {
        let req = TestRequest::default()
            .uri("/message?token=abc123")
            .to_http_request();

        let token = extract_token(&req);

        assert_eq!(token, Some("abc123"));
    }

    #[test]
    fn returns_none_when_token_missing() {
        let req = TestRequest::default()
            .uri("/message")
            .to_http_request();

        let token = extract_token(&req);

        assert_eq!(token, None);
    }
}

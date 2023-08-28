use tracing::error;

pub fn extract_bearer_token(header_value: &actix_web::http::header::HeaderValue) -> Option<String> {
    let prefix = "Bearer ";

    if header_value.is_empty() {
        error!("Header value must not be empty in order to acquire bearer token");
        return None;
    }

    if header_value.len() <= prefix.len() {
        error!("Header value must be longer than the Bearer prefix");
        return None;
    }

    let Ok(value_string) = header_value.to_str() else {
        error!("Header value must be representable as a string");
        return None;
    };

    Some(value_string[prefix.len()..].to_owned())
}

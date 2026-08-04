use axum::http::HeaderMap;
use serde::Deserialize;
use serde_json::{from_value, Value};

#[derive(Deserialize, Debug, Clone)]
pub struct Tokens {
    pub access_token: String,
    pub id_token: String,
}

#[derive(Debug, Clone)]
pub struct Token(pub String);

fn find_token(headers: &HeaderMap, token_name: &str) -> Option<Token> {
    headers
        .get(token_name)
        .and_then(|value| value.to_str().ok())
        .and_then(|s| {
            if s.starts_with("Bearer ") {
                Some(Token(s["Bearer ".len()..].to_string()))
            } else {
                Some(Token(s.to_string()))
            }
        })
}

pub fn get_access_token(headers: &HeaderMap) -> Option<Token> {
    find_token(headers, "Access-Token")
}

pub fn get_id_token(headers: &HeaderMap) -> Option<Token> {
    find_token(headers, "ID-Token")
}

// pub fn get_tokens(headers: &HeaderMap) -> Option<Tokens> {
//     let access_token = get_access_token(headers)?.0;
//     let id_token = get_id_token(headers)?.0;
//     Some(Tokens {
//         access_token,
//         id_token
//     })
// }

pub async fn get_subscription_tokens(value: Value) -> Result<Tokens, String> {
    match from_value::<Tokens>(value) {
        Ok(payload) => Ok(payload),
        Err(_) => Err("Token is required".into()),
    }
}

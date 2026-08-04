use std::{
    error::Error,
    fmt::{Display, Formatter, Result},
};

#[derive(Debug)]
pub enum KeycloakError {
    ConnectionError(reqwest::Error),
    ResponseError(String),
    OtherwiseError(String),
    MissingEnvVar(&'static str),
    ParsingError(serde_json::Error),
    AuthenticationError(String),
}

impl Display for KeycloakError {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        match self {
            KeycloakError::MissingEnvVar(var) => write!(f, "Missing environment variable: {}", var),
            KeycloakError::ConnectionError(err) => write!(f, "Connection error: {}", err),
            KeycloakError::OtherwiseError(msg) => write!(f, "Other error: {}", msg),
            KeycloakError::ResponseError(msg) => write!(f, "Response error: {}", msg),
            KeycloakError::ParsingError(err) => write!(f, "Parsing error: {}", err),
            KeycloakError::AuthenticationError(msg) => write!(f, "Authentication error: {}", msg),
        }
    }
}

impl Error for KeycloakError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            KeycloakError::ConnectionError(err) => Some(err),
            KeycloakError::ResponseError(_) => None,
            KeycloakError::MissingEnvVar(_) => None,
            KeycloakError::ParsingError(err) => Some(err),
            KeycloakError::AuthenticationError(_) => None,
            KeycloakError::OtherwiseError(_) => None,
        }
    }
}

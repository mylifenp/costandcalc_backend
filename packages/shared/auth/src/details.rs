use dotenvy::dotenv;
use jsonwebtoken::{
    Algorithm, DecodingKey, Validation, decode,
    errors::{Error as JwtError, ErrorKind},
};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, env};

fn get_env_value(key: &str) -> Option<String> {
    dotenv().ok(); // Load environment variables from .env file
    env::var(key).ok() // Retrieve the value for the given key
}

pub type Result<T> = std::result::Result<T, AuthError>;

#[derive(Debug)]
pub enum AuthError {
    InvalidToken,
    InvalidIssuer,
    ExpiredSignature,
    InvalidSubject,
    InvalidKeyFormat,
    InvalidAlgorithmName,
    InvalidAlgorithm,
    JwtError(JwtError),
}

impl From<JwtError> for AuthError {
    fn from(err: JwtError) -> Self {
        match *err.kind() {
            ErrorKind::InvalidToken => AuthError::InvalidToken,
            ErrorKind::InvalidIssuer => AuthError::InvalidIssuer,
            ErrorKind::ExpiredSignature => AuthError::ExpiredSignature,
            ErrorKind::InvalidSubject => AuthError::InvalidSubject,
            ErrorKind::InvalidKeyFormat => AuthError::InvalidKeyFormat,
            ErrorKind::InvalidAlgorithmName => AuthError::InvalidAlgorithmName,
            ErrorKind::InvalidAlgorithm => AuthError::InvalidAlgorithm,
            _ => AuthError::JwtError(err),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct OrganizationId {
    pub id: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct RealmAccess {
    pub roles: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub aud: String,
    pub sub: String,
    pub exp: u64,
    #[serde(default)]
    pub email_verified: bool,
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub preferred_username: String,
    #[serde(default)]
    pub given_name: String,
    #[serde(default)]
    pub family_name: String,
    #[serde(default)]
    pub email: String,
    #[serde(default)]
    pub locale: String,
    pub organization: Option<String>,
    #[serde(rename = "organization_ids")]
    pub organization_ids: Option<std::collections::HashMap<String, OrganizationId>>,
    pub realm_access: Option<RealmAccess>,
}

pub fn decode_token(token: &str) -> Result<Claims> {
    if let Some(jwt_secret) = get_env_value("JWT_SECRET") {
        let mut validation = Validation::new(Algorithm::RS256);

        let issuer = get_env_value("ISSUER")
            .unwrap_or_else(|| "https://auth.datenium.de/realms/cost_and_calc".to_string());
        let issuer_id =
            get_env_value("ISSUER_ID").unwrap_or_else(|| "cost_and_calc_id".to_string());

        validation.set_audience(&[&issuer_id]);
        validation.set_issuer(&[&issuer]);
        validation.validate_aud = true;
        validation.required_spec_claims = HashSet::new();

        let decoding_key = match DecodingKey::from_rsa_pem(jwt_secret.as_bytes()) {
            Ok(key) => key,
            Err(e) => {
                // eprintln!("KEY_PARSE_ERROR: {:?} - {}", e.kind(), e);
                return Err(AuthError::from(e));
            }
        };

        match decode::<Claims>(token, &decoding_key, &validation) {
            Ok(token_data) => {
                // println!("TOKEN_DATA: {:?}", token_data);
                Ok(token_data.claims)
            }
            Err(e) => {
                // eprintln!("TOKEN_DECODE_ERROR: kind={:?}, msg={}", e.kind(), e);
                Err(AuthError::from(e))
            }
        }
    } else {
        // eprintln!("TOKEN_ERROR: JWT_SECRET not set");
        Err(AuthError::InvalidKeyFormat)
    }
}

impl Claims {
    pub fn roles(&self) -> Vec<String> {
        self.realm_access
            .as_ref()
            .map(|ra| ra.roles.clone())
            .unwrap_or_default()
    }
    pub fn organization_ids(&self) -> Vec<String> {
        match &self.organization_ids {
            Some(org_ids_map) => org_ids_map
                .values()
                .map(|org_id| org_id.id.clone())
                .collect(),
            None => vec![],
        }
    }
}

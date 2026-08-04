use dotenvy::dotenv;
use jsonwebtoken::{
    decode,
    errors::{Error as JwtError, ErrorKind},
    Algorithm, DecodingKey, Validation,
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
    pub email_verified: bool,
    pub name: String,
    pub preferred_username: String,
    pub given_name: String,
    pub family_name: String,
    pub email: String,
    pub locale: String,
    pub organization: Option<String>,
    #[serde(rename = "organization_ids")]
    pub organization_ids: Option<std::collections::HashMap<String, OrganizationId>>,
    pub realm_access: Option<RealmAccess>,
}

pub fn decode_token(token: &str) -> Result<Claims> {
    if let Some(jwt_secret) = get_env_value("JWT_SECRET") {
        let issuer = get_env_value("ISSUER")
            .unwrap_or_else(|| "https://auth.datenium.de/realms/meatone".to_string());
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&["meatone_id"]);
        validation.set_issuer(&[&issuer]);
        validation.validate_aud = false;
        validation.required_spec_claims = HashSet::new();

        let token_data = decode::<Claims>(
            &token,
            &DecodingKey::from_rsa_pem(jwt_secret.as_bytes())?,
            &validation,
        )?;

        Ok(token_data.claims)
    } else {
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

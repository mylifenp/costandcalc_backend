// filepath: /keycloak-client/keycloak-client/src/types.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct KeycloakToken {
    pub access_token: String,
    pub expires_in: Option<u64>,
    pub refresh_expires_in: Option<u64>,
    pub refresh_token: Option<String>,
    pub token_type: Option<String>,
    pub id_token: Option<String>,
    pub not_before_policy: Option<u64>,
    pub session_state: Option<String>,
    pub scope: Option<String>,
    pub token_created: Option<u64>, // Unix timestamp when the token was created
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakUser {
    pub id: String,
    pub username: String,
    pub email: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct KeycloakOrganization {
    pub id: String,
    pub name: String,
    pub alias: String,
    pub enabled: bool,
    pub description: String,
    pub domains: Vec<Domain>,
}

#[derive(Serialize)]
pub struct OrganizationBody {
    pub name: String,
    pub description: String,
    pub alias: String,
    pub domains: Vec<Domain>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Domain {
    pub name: String,
    pub verified: Option<bool>,
}

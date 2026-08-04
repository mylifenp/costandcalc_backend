use reqwest::Client;
use std::{
    collections::HashMap,
    env,
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use tokio::sync::Mutex;

use crate::keycloak::{
    error::KeycloakError,
    types::{Domain, KeycloakOrganization, KeycloakToken, OrganizationBody},
};

pub struct KeycloakClient {
    client: Client,
    base_url: String,
    admin_url: String,
    client_id: String,
    client_secret: String,
    token: Arc<Mutex<Option<KeycloakToken>>>,
}

impl KeycloakClient {
    pub fn new() -> Result<Self, KeycloakError> {
        let keycloak_realm = env::var("KEYCLOAK_REALM")
            .map_err(|_| KeycloakError::MissingEnvVar("KEYCLOAK_REALM"))?;
        let keycloak_url =
            env::var("KEYCLOAK_URL").map_err(|_| KeycloakError::MissingEnvVar("KEYCLOAK_URL"))?;
        let client_id = env::var("KEYCLOAK_CLIENT_ID")
            .map_err(|_| KeycloakError::MissingEnvVar("KEYCLOAK_CLIENT_ID"))?;
        let client_secret = env::var("KEYCLOAK_CLIENT_SECRET")
            .map_err(|_| KeycloakError::MissingEnvVar("KEYCLOAK_CLIENT_SECRET"))?;

        let client = Client::builder()
            .build()
            .map_err(KeycloakError::ConnectionError)?;

        let base_url = format!(
            "{}/realms/{}",
            keycloak_url.trim_end_matches('/'),
            keycloak_realm
        );

        let admin_url = format!(
            "{}/admin/realms/{}",
            keycloak_url.trim_end_matches('/'),
            keycloak_realm
        );

        Ok(KeycloakClient {
            client,
            base_url,
            admin_url,
            client_id,
            client_secret,
            token: Arc::new(Mutex::new(None)),
        })
    }

    pub async fn fetch_and_store_token(&self) -> Result<(), KeycloakError> {
        let username = env::var("SUPER_USER").expect("SUPER_USER must be set");
        let password = env::var("KEYCLOAK_PASSWORD").expect("KEYCLOAK_PASSWORD must be set");
        let scope = env::var("KEYCLOAK_SCOPE").unwrap_or_else(|_| "openid".into());
        let grant_type = env::var("KEYCLOAK_GRANT_TYPE").unwrap_or_else(|_| "password".into());
        let token_url = format!(
            "{}/protocol/openid-connect/token",
            self.base_url.trim_end_matches('/'),
        );

        let mut params = HashMap::new();
        params.insert("grant_type", &grant_type);
        params.insert("client_id", &self.client_id);
        params.insert("client_secret", &self.client_secret);
        params.insert("username", &username);
        params.insert("password", &password);
        params.insert("scope", &scope);

        let resp = self
            .client
            .post(token_url)
            .form(&params)
            .send()
            .await
            .map_err(KeycloakError::ConnectionError)?;

        let status = resp.status();
        let body = resp.text().await.map_err(KeycloakError::ConnectionError)?;

        if !status.is_success() {
            return Err(KeycloakError::OtherwiseError(format!(
                "Failed to get token: {}",
                body
            )));
        }

        let token: KeycloakToken = serde_json::from_str(&body)
            .map_err(|e| KeycloakError::OtherwiseError(format!("Token parse error: {}", e)))?;

        let mut guard = self.token.lock().await;
        *guard = Some(token);

        Ok(())
    }

    pub async fn get_token(&self) -> Option<KeycloakToken> {
        let mut guard = self.token.lock().await;

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|d| d.as_secs())
            .unwrap_or(0);

        let expired = guard
            .as_ref()
            .and_then(|token| token.expires_in)
            .map(|expires_in| {
                // You may want to store the token's creation time in KeycloakToken for more accuracy.
                // Here, we assume the token was created at the time of fetch_and_store_token.
                // For now, always refresh if expires_in is present and less than now.
                let token_created = guard.as_ref().and_then(|t| t.token_created).unwrap_or(now); // Add token_created: Option<u64> to KeycloakToken for accuracy.
                now > token_created + expires_in - 30 // 30s buffer
            })
            .unwrap_or(true);

        if guard.is_none() || expired {
            drop(guard); // Release lock before await
            let _ = self.fetch_and_store_token().await;
            guard = self.token.lock().await;
        }

        guard.as_ref().cloned()
    }

    // pub async fn post<T: serde::Serialize>(
    //     &self,
    //     endpoint: &str,
    //     body: T,
    //     is_admin: bool,
    // ) -> Result<String, KeycloakError> {
    //     let url = if is_admin {
    //         format!("{}/{}", self.admin_url, endpoint)
    //     } else {
    //         format!("{}/{}", self.base_url, endpoint)
    //     };
    //     println!("POST URL: {}", url);

    //     let access_token = self
    //         .get_token()
    //         .await
    //         .and_then(|token| token.access_token.into());

    //     let mut req = self.client.post(&url);
    //     if let Some(token) = access_token {
    //         req = req.bearer_auth(token);
    //     }

    //     let response = req
    //         .json(&body)
    //         .send()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;

    //     let body = response
    //         .text()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;

    //     println!("POST Response Body: {}", body);

    //     Ok(body)
    // }

    // pub async fn put(
    //     &self,
    //     endpoint: &str,
    //     params: HashMap<&str, &str>,
    // ) -> Result<String, KeycloakError> {
    //     let url = format!("{}/{}", self.base_url, endpoint);
    //     let response = self
    //         .client
    //         .put(&url)
    //         .form(&params)
    //         .send()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;
    //     let body = response
    //         .text()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;
    //     Ok(body)
    // }

    // pub async fn get(&self, endpoint: &str, is_admin: bool) -> Result<String, KeycloakError> {
    //     let url = if is_admin {
    //         format!("{}/{}", self.admin_url, endpoint)
    //     } else {
    //         format!("{}/{}", self.base_url, endpoint)
    //     };

    //     let access_token = self
    //         .get_token()
    //         .await
    //         .and_then(|token| token.access_token.into());

    //     let mut req = self.client.get(&url);
    //     if let Some(token) = access_token {
    //         req = req.bearer_auth(token);
    //     }

    //     let response = req.send().await.map_err(KeycloakError::ConnectionError)?;

    //     let body = response
    //         .text()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;
    //     Ok(body)
    // }

    // pub async fn delete(&self, endpoint: &str) -> Result<String, KeycloakError> {
    //     let url = format!("{}/{}", self.base_url, endpoint);
    //     let response = self
    //         .client
    //         .delete(&url)
    //         .send()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;
    //     let body = response
    //         .text()
    //         .await
    //         .map_err(KeycloakError::ConnectionError)?;
    //     Ok(body)
    // }

    pub async fn get_users() -> Result<(), KeycloakError> {
        // Implement user fetching logic here
        Ok(())
    }

    pub async fn create_organization(
        &self,
        name: &String,
        description: &Option<String>,
    ) -> Result<String, KeycloakError> {
        let url = format!("{}/organizations", self.admin_url);

        let desc = match description {
            Some(description) => description.to_string(),
            _ => "".to_string(),
        };

        let access_token = self
            .get_token()
            .await
            .and_then(|token| token.access_token.into());

        let mut req = self.client.post(&url);
        if let Some(token) = access_token {
            req = req.bearer_auth(token);
        }

        let alias = name.clone().to_lowercase().replace(" ", "-");
        let body = OrganizationBody {
            name: name.clone(),
            description: desc.clone(),
            alias: alias.clone(),
            domains: vec![Domain {
                name: format!("{}.auth.datenium.de", alias.clone()),
                verified: Some(false),
            }],
        };

        let res = req
            .json(&body)
            .send()
            .await
            .map_err(KeycloakError::ConnectionError)?;

        if !res.status().is_success() {
            Err(KeycloakError::OtherwiseError(format!(
                "Failed to create organization {}",
                name
            )))
        } else {
            Ok(format!("{} Organization created Successfully", name))
        }
    }

    pub async fn get_organizations(&self) -> Result<Vec<KeycloakOrganization>, KeycloakError> {
        let url = format!("{}/organizations", self.admin_url);

        let access_token = self
            .get_token()
            .await
            .and_then(|token| token.access_token.into());

        let mut req = self.client.get(&url);
        if let Some(token) = access_token {
            req = req.bearer_auth(token);
        }

        let res = req.send().await.map_err(KeycloakError::ConnectionError)?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(KeycloakError::OtherwiseError(format!(
                "Failed to get organizations: {:?}",
                body
            )));
        }

        let body: Vec<KeycloakOrganization> =
            res.json().await.map_err(KeycloakError::ConnectionError)?;

        Ok(body)
    }

    pub async fn delete_organization(&self, id: String) -> Result<String, KeycloakError> {
        let url = format!("{}/organizations/{}", self.admin_url, id);

        let access_token = self
            .get_token()
            .await
            .and_then(|token| token.access_token.into());

        let mut req = self.client.delete(&url);
        if let Some(token) = access_token {
            req = req.bearer_auth(token);
        }

        let res = req.send().await.map_err(KeycloakError::ConnectionError)?;

        if !res.status().is_success() {
            let body = res.text().await.unwrap_or_default();
            return Err(KeycloakError::OtherwiseError(format!(
                "Failed to get organizations: {}",
                body
            )));
        } else {
            Ok(format!("Organization with id {} deleted", id))
        }
    }

    pub async fn add_organization(
        &self,
        name: &String,
        description: &Option<String>,
    ) -> Result<KeycloakOrganization, KeycloakError> {
        self.create_organization(name, description).await?;

        let orgs = self.get_organizations().await?;
        let org = orgs.into_iter().find(|org| org.name == name.to_string());

        match org {
            Some(org) => Ok(org),
            None => Err(KeycloakError::OtherwiseError(format!(
                "Organization '{}' not found after creation",
                name
            ))),
        }
    }
}

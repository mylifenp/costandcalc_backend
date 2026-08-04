use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::IdentityHealthStatus;

#[derive(Default)]
pub struct IdentityHealthQuery;

#[Object]
impl IdentityHealthQuery {
    async fn identity_health(&self) -> Result<IdentityHealthStatus> {
        Ok(IdentityHealthStatus::status().await)
    }
}
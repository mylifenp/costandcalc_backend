use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::AdminHealthStatus;

#[derive(Default)]
pub struct AdminHealthQuery;

#[Object]
impl AdminHealthQuery {
    async fn admin_health(&self) -> Result<AdminHealthStatus> {
        Ok(AdminHealthStatus::status().await)
    }
}
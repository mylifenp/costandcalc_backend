use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::AssetsHealthStatus;

#[derive(Default)]
pub struct AssetsHealthQuery;

#[Object]
impl AssetsHealthQuery {
    async fn assets_health(&self) -> Result<AssetsHealthStatus> {
        Ok(AssetsHealthStatus::status().await)
    }
}
use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::EstimationHealthStatus;

#[derive(Default)]
pub struct EstimationHealthQuery;

#[Object]
impl EstimationHealthQuery {
    async fn estimation_health(&self) -> Result<EstimationHealthStatus> {
        Ok(EstimationHealthStatus::status().await)
    }
}
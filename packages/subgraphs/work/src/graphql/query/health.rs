use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::WorkHealthStatus;

#[derive(Default)]
pub struct WorkHealthQuery;

#[Object]
impl WorkHealthQuery {
    async fn work_health(&self) -> Result<WorkHealthStatus> {
        Ok(WorkHealthStatus::status().await)
    }
}
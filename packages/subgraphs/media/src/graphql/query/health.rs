use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::MediaHealthStatus;

#[derive(Default)]
pub struct MediaHealthQuery;

#[Object]
impl MediaHealthQuery {
    async fn media_health(&self) -> Result<MediaHealthStatus> {
        Ok(MediaHealthStatus::status().await)
    }
}
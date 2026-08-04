use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::NotificationHealthStatus;

#[derive(Default)]
pub struct NotificationHealthQuery;

#[Object]
impl NotificationHealthQuery {
    async fn notification_health(&self) -> Result<NotificationHealthStatus> {
        Ok(NotificationHealthStatus::status().await)
    }
}
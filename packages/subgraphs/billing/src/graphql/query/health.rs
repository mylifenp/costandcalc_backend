use async_graphql::{Object, Result};

use crate::graphql::schema::health_status::BillingHealthStatus;

#[derive(Default)]
pub struct BillingHealthQuery;

#[Object]
impl BillingHealthQuery {
    async fn billing_health(&self) -> Result<BillingHealthStatus> {
        Ok(BillingHealthStatus::status().await)
    }
}
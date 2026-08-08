use async_graphql::{Context, Object, Result};
use shared::details::Claims;

use crate::graphql::schema::health_status::AdminHealthStatus;

#[derive(Default)]
pub struct AdminHealthQuery;

#[Object]
impl AdminHealthQuery {
    async fn admin_health(&self, ctx: &Context<'_>) -> Result<AdminHealthStatus> {
        let me = ctx.data::<Option<Claims>>()?;
        if me.is_none() {
            return Err(async_graphql::Error::new("Unauthorized"));
        }
        let email = me.as_ref().map(|c| c.email.clone()).unwrap_or_default();
        Ok(AdminHealthStatus::status(email).await)
    }
}

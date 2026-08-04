pub mod user_assertions;

use std::sync::Arc;

use async_graphql::{Context, Result};
use backend_libs_auth::details::Claims;
use backend_libs_cache::RedisCache;
use order_db::models::user::ProducerUserDocument;


pub async fn fetch_user_from_claims(ctx: &Context<'_>) -> Result<Option<ProducerUserDocument>> {
    let claims = ctx.data_opt::<Option<Claims>>().ok_or("Claims not found")?;
    let repos = ctx.data::<Arc<Repositories>>()?;

    if claims.is_none() {
        return Ok(None);
    }
    let claims = claims.as_ref().unwrap();
    if claims.email.is_empty() {
        return Ok(None);
    }
    let cache = ctx.data::<RedisCache>()?;
    let cache_key = format!("producer_user:{}", claims.email);

    if let Some(cached_user) = cache.get_json::<ProducerUserDocument>(&cache_key).await? {
        return Ok(Some(cached_user));
    }
    let user = repos.producer_user_repo.get_by_email(&claims.email).await?;
    if let Some(ref user_doc) = user {
        cache
            .set_json(&cache_key, user_doc, 300) // Cache for 5 minutes
            .await?;
    }

    Ok(user)
}

pub async fn fetch_producer_from_claims(ctx: &Context<'_>) -> Result<Option<String>> {
    let user = fetch_user_from_claims(ctx).await?;
    if let Some(user_doc) = user {
        Ok(Some(user_doc.producer_id.to_hex()))
    } else {
        Ok(None)
    }
}

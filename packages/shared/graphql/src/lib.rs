use async_graphql::{http::GraphiQLSource, Data, Result};
use axum::response::{Html, IntoResponse};

use shared_auth::{
    details::decode_token,
    tokens::{get_subscription_tokens, Tokens},
};
use serde_json::Value;

pub async fn graphiql_handler() -> impl IntoResponse {
    Html(
        GraphiQLSource::build()
            .endpoint("/graphql")
            .subscription_endpoint("/subscriptions")
            .finish(),
    )
}

pub async fn on_subscription_init(value: Value) -> Result<Data> {
    match get_subscription_tokens(value).await {
        Ok(payload) => {
            let access_token = payload.access_token;
            let id_token = payload.id_token;
            let mut data = Data::default();
            let me = decode_token(&id_token).ok();
            data.insert(me);
            data.insert(Tokens {
                access_token: access_token.clone(),
                id_token: id_token.clone(),
            });
            Ok(data)
        }
        Err(err) => Err(err.into()),
    }
}

use async_graphql::http::ALL_WEBSOCKET_PROTOCOLS;
use async_graphql_axum::{GraphQLProtocol, GraphQLRequest, GraphQLResponse, GraphQLWebSocket};
use axum::{
    extract::{ws::WebSocketUpgrade, State},
    http::HeaderMap,
    response::Response,
};
use shared::{details::{Claims, decode_token}, on_subscription_init, tokens::{Tokens, get_access_token, get_id_token}};

use crate::graphql::schema::schema::AppSchema;



pub async fn graphql_handler(
    State(schema): State<AppSchema>,
    headers: HeaderMap,
    req: GraphQLRequest,
) -> GraphQLResponse {
    let mut req = req.into_inner();
    let access_token = get_access_token(&headers).map(|t| t.0).unwrap_or_default();
    let id_token = get_id_token(&headers).map(|t| t.0).unwrap_or_default();

    if access_token.is_empty() && id_token.is_empty() {
        req = req.data::<Option<Claims>>(None);
        return schema.execute(req).await.into();
    }

    req = req.data(Tokens {
        access_token: access_token.clone(),
        id_token: id_token.clone(),
    });

    let me = decode_token(&id_token).ok();
    req = req.data::<Option<Claims>>(me.clone());

    schema.execute(req).await.into()
}

pub async fn graphql_ws_handler(
    State(schema): State<AppSchema>,
    protocol: GraphQLProtocol,
    websocket: WebSocketUpgrade,
) -> Response {
    websocket
        .protocols(ALL_WEBSOCKET_PROTOCOLS)
        .on_upgrade(move |stream| {
            GraphQLWebSocket::new(stream, schema.clone(), protocol)
                .on_connection_init(on_subscription_init)
                .serve()
        })
}

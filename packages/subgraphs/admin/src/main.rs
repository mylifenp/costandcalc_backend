mod utils;
mod graphql;
mod repositories;
use std::env;

use axum::{Router, routing::get, serve};
#[cfg(debug_assertions)]
use dotenvy::dotenv;
use shared::graphiql_handler;
use tokio::net::TcpListener;
use tower_http::{
    cors::{self, CorsLayer},
};

use crate::{graphql::schema::schema::{AppSchema, build_schema}, utils::{graphql_handler, graphql_ws_handler}};


#[tokio::main]
async fn main() {
    #[cfg(debug_assertions)]
    dotenv().ok();

    // host should always be 0.0.0.0 for docker
    let host = env::var("ADMIN_HOST").unwrap_or_else(|_| "0.0.0.0".to_string());
    let port = env::var("ADMIN_PORT").unwrap_or_else(|_| "8001".to_string());

    let cors = CorsLayer::new()
        .allow_origin(cors::Any)
        .allow_methods(cors::Any)
        .allow_headers(cors::Any);

    let schema: AppSchema = build_schema().await;

    let app = Router::new()
        .route("/graphql", get(graphiql_handler).post(graphql_handler))
        .route("/subscriptions", get(graphql_ws_handler))
        .layer(cors)
        .with_state(schema.clone());


    println!("Admin playground: http://{host}:{port}/graphql");
    
    let listener = TcpListener::bind(format!("{host}:{port}")).await.unwrap();
    serve(listener, app).await.unwrap();
}

use async_graphql::{EmptyMutation, EmptySubscription, Schema};
// use dotenvy::dotenv;
use shared::keycloak::client::KeycloakClient;

use crate::graphql::query::Query;

pub type AppSchema = Schema<Query, EmptyMutation, EmptySubscription>;

pub async fn build_schema() -> AppSchema {
    // dotenv().ok();

    // let mongo_uri =
    //     env::var("ORDER_MONGODB_URI").expect("ORDER_MONGODB_URI must be set in .env file");
    // // Initialize MongoDB
    // init_db(mongo_uri).await.expect("MongoDB init failed");

    // let pubsub_url =
    //     env::var("REDIS_PUBSUB_URL").expect("REDIS_PUBSUB_URL must be set in .env file");
    // let redis_url = env::var("REDIS_URL").expect("REDIS_URL must be set in .env file");

    // let redis_pubsub = init_pubsub(pubsub_url).expect("RedisPubSub init failed");
    // let cache =
    //     init_cache(redis_url, "order_cache").expect("RedisCache init failed");

    // let emailer = init_emailer();
    let keycloak_client = KeycloakClient::new().expect("Failed to create Keycloak client");
    // let file_uploader = init_file_uploader();

    // let repositories = Arc::new(
    //     repositories::init_repos()
    //         .await
    //         .expect("MongoDB init failed"),
    // );

    let schema = Schema::build(
        Query::default(),
        EmptyMutation,
        EmptySubscription,
    );

    schema
        // .data(cache)
        // .data(repositories)
        // .data(redis_pubsub)
        // .data(file_uploader)
        // .data(emailer)
        .data(keycloak_client)
        // .extension(UserExtension)
        .enable_federation()
        .enable_subscription_in_federation()
        .finish()
}
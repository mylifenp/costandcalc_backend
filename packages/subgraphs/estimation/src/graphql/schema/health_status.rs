use async_graphql::SimpleObject;


#[derive(SimpleObject)]
pub struct EstimationHealthStatus {
    pub api_status: String,
    // pub db_status: String,
}
impl EstimationHealthStatus {
    pub async fn status() -> Self {
        // let db_status = mongo_ping()
        //     .await
        //     .map(|_| "OK".to_string())
        //     .unwrap_or_else(|_| "ERROR".to_string());

        Self {
            api_status: "OK".to_string(),
            // db_status,
        }
    }
}
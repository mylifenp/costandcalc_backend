use async_graphql::SimpleObject;

#[derive(SimpleObject)]
pub struct AdminHealthStatus {
    pub api_status: String,
    // pub db_status: String,
    pub email: String,
}
impl AdminHealthStatus {
    pub async fn status(email: String) -> Self {
        // let db_status = mongo_ping()
        //     .await
        //     .map(|_| "OK".to_string())
        //     .unwrap_or_else(|_| "ERROR".to_string());

        Self {
            email,
            api_status: "OK".to_string(),
            // db_status,
        }
    }
}

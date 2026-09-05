use std::env;

use minio::s3::{
    MinioClient, MinioClientBuilder,
    creds::StaticProvider,
    http::BaseUrl,
    response::BucketExistsResponse,
    types::{BucketName, S3Api},
};

use crate::utils::uploader::file_uploader::FileUploader;

pub mod file_type;
pub mod file_uploader;

pub async fn init_file_uploader() -> FileUploader {
    let folder_name = env::var("STATIC_FOLDER_NAME").unwrap_or_else(|_| "static".to_string());
    let endpoint = env::var("S3_ENDPOINT").unwrap_or_else(|_| "http://localhost:9000".to_string());
    let public_base_url = env::var("S3_PUBLIC_BASE_URL").unwrap_or_else(|_| endpoint.clone());
    let bucket = env::var("S3_BUCKET").unwrap_or_else(|_| "costandcalc".to_string());
    let access_key = env::var("S3_ACCESS_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let secret_key = env::var("S3_SECRET_KEY").unwrap_or_else(|_| "minioadmin".to_string());
    let public_read = env::var("S3_PUBLIC_READ")
        .map(|value| value.eq_ignore_ascii_case("true") || value == "1")
        .unwrap_or(true);

    let base_url = endpoint
        .parse::<BaseUrl>()
        .expect("S3_ENDPOINT must be a valid URL");

    let static_provider = StaticProvider::new(&access_key, &secret_key, None);
    let client = MinioClientBuilder::new(base_url)
        .provider(Some(static_provider))
        .build()
        .expect("Failed to initialize MinIO client");

    ensure_bucket_exists(&client, &bucket)
        .await
        .expect("Failed to ensure MinIO bucket exists");

    if public_read {
        ensure_public_read_policy(&client, &bucket, &folder_name)
            .await
            .expect("Failed to apply public read policy to MinIO bucket");
    }

    FileUploader::new(client, bucket, folder_name, public_base_url)
}

async fn ensure_bucket_exists(
    client: &MinioClient,
    bucket: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bucket_name = BucketName::new(bucket)?;
    let response: BucketExistsResponse = client
        .bucket_exists(bucket_name.clone())?
        .build()
        .send()
        .await?;

    if !response.exists() {
        client.create_bucket(bucket_name)?.build().send().await?;
    }

    Ok(())
}

async fn ensure_public_read_policy(
    client: &MinioClient,
    bucket: &str,
    folder_name: &str,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let bucket_name = BucketName::new(bucket)?;
    let resource = public_resource_arn(bucket, folder_name);
    let policy = format!(
        r#"{{
            "Version": "2012-10-17",
            "Statement": [
                {{
                "Effect": "Allow",
                "Principal": "*",
                "Action": ["s3:GetObject"],
                "Resource": ["{}"]
                }}
            ]
        }}"#,
        resource
    );

    client
        .put_bucket_policy(&bucket_name)?
        .config(policy)
        .build()
        .send()
        .await?;

    Ok(())
}

fn public_resource_arn(bucket: &str, folder_name: &str) -> String {
    let prefix = folder_name.trim_matches('/');
    if prefix.is_empty() {
        format!("arn:aws:s3:::{}/*", bucket)
    } else {
        format!("arn:aws:s3:::{}/{}/*", bucket, prefix)
    }
}

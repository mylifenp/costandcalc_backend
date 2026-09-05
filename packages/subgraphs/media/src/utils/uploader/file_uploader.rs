use futures_util::StreamExt;
use minio::s3::{
    MinioClient,
    builders::ObjectContent,
    types::{S3Api, ToStream},
};
use nanoid::nanoid;
use std::{error::Error, io::Read, path::Path};

pub struct FileUploader {
    client: MinioClient,
    bucket: String,
    folder_name: String,
    public_base_url: String,
}

impl FileUploader {
    pub fn new(
        client: MinioClient,
        bucket: String,
        folder_name: String,
        public_base_url: String,
    ) -> Self {
        Self {
            client,
            bucket,
            folder_name,
            public_base_url,
        }
    }

    fn build_object_key(&self, id: &str, extension: &str) -> String {
        let prefix = self.folder_name.trim_matches('/');
        if prefix.is_empty() {
            format!("{}{}", id, extension)
        } else {
            format!("{}/{}{}", prefix, id, extension)
        }
    }

    fn build_public_url(&self, object_key: &str) -> String {
        format!(
            "{}/{}/{}",
            self.public_base_url.trim_end_matches('/'),
            self.bucket,
            object_key
        )
    }

    pub async fn upload(
        &self,
        filename: String,
        mut content: impl Read,
    ) -> Result<(String, String), Box<dyn Error>> {
        let id = nanoid!();

        let extension = Path::new(&filename)
            .extension()
            .and_then(|ext| ext.to_str())
            .map(|ext| format!(".{}", ext))
            .unwrap_or_default();

        let object_key = self.build_object_key(&id, &extension);

        let mut buffer = Vec::new();
        Read::read_to_end(&mut content, &mut buffer)?;

        let content = ObjectContent::from(buffer);
        self.client
            .put_object_content(&self.bucket, &object_key, content)?
            .build()
            .send()
            .await?;

        let url = self.build_public_url(&object_key);

        Ok((id, url))
    }

    pub async fn upload_multiple(
        &self,
        files: Vec<(String, impl Read)>,
    ) -> Result<Vec<(String, String)>, Box<dyn Error>> {
        let mut results = Vec::new();

        for (filename, content) in files {
            match self.upload(filename, content).await {
                Ok(result) => results.push(result),
                Err(e) => {
                    eprintln!("Failed to upload file:{}", e);
                    return Err(e);
                }
            }
        }

        Ok(results)
    }

    pub async fn delete(&self, id: &str) -> Result<(), Box<dyn Error>> {
        if !Self::is_valid_nanoid(id) {
            return Err(format!("Invalid file ID: {}", id).into());
        }

        let prefix = self.build_object_key(id, "");
        let mut object_keys = Vec::new();

        let mut stream = self
            .client
            .list_objects(&self.bucket)?
            .prefix(prefix.clone())
            .recursive(true)
            .use_api_v1(false)
            .build()
            .to_stream()
            .await;

        while let Some(page) = stream.next().await {
            let page = page?;
            for entry in page.contents {
                if !entry.is_prefix && entry.name.starts_with(&prefix) {
                    object_keys.push(entry.name);
                }
            }
        }

        if object_keys.is_empty() {
            return Err(format!("File with ID {} not found", id).into());
        }

        for key in object_keys {
            self.client
                .delete_object(&self.bucket, key)?
                .build()
                .send()
                .await?;
        }

        Ok(())
    }

    pub async fn delete_multiple(&self, ids: &[String]) -> Result<(), Box<dyn Error>> {
        for id in ids {
            self.delete(id).await?;
        }
        Ok(())
    }

    fn is_valid_nanoid(id: &str) -> bool {
        // Accept 10-30 character IDs (flexible)
        let len = id.len();
        len >= 10
            && len <= 30
            && id
                .chars()
                .all(|c| c.is_ascii_alphanumeric() || c == '_' || c == '-')
    }
}

// For custom alphabet/size validation

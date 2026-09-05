use async_graphql::{Context, Error, ID, Object, Result, Upload};

use crate::{graphql::schema::file::FileInfo, utils::uploader::file_uploader::FileUploader};

#[derive(Default)]
pub struct FileMutation;

#[Object]
impl FileMutation {
    async fn upload_file(&self, ctx: &Context<'_>, file: Upload) -> Result<FileInfo> {
        let stream = file.value(ctx)?;
        let filename = stream.filename.clone();
        let file_uploader = ctx.data::<FileUploader>()?;

        let (id, url) = file_uploader
            .upload(filename, stream.content)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        Ok(FileInfo { id: ID(id), url })
    }

    async fn upload_files(&self, ctx: &Context<'_>, files: Vec<Upload>) -> Result<Vec<FileInfo>> {
        let file_uploader = ctx.data::<FileUploader>()?;

        let mut upload_inputs = Vec::new();
        for file in files {
            let stream = file.value(ctx)?;
            upload_inputs.push((stream.filename.clone(), stream.content));
        }

        let results = file_uploader
            .upload_multiple(upload_inputs)
            .await
            .map_err(|e| Error::new(e.to_string()))?;

        let file_infos = results
            .into_iter()
            .map(|(id, url)| FileInfo { id: ID(id), url })
            .collect();

        Ok(file_infos)
    }

    async fn delete_files(&self, ctx: &Context<'_>, ids: Vec<ID>) -> Result<Vec<ID>> {
        let file_uploader = ctx.data::<FileUploader>()?;

        let string_ids: Vec<String> = ids.iter().map(|id| id.to_string()).collect();
        file_uploader
            .delete_multiple(&string_ids)
            .await
            .map_err(|e| Error::new(e.to_string()))?;
        Ok(ids)
    }
}

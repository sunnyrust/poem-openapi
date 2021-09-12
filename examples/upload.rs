use std::collections::HashMap;

use poem::{listener::TcpListener, Error, Result};
use poem_openapi::{
    payload::Json, types::multipart::Upload, Multipart, Object, OpenApi, OpenApiService, Response,
};
use tokio::sync::Mutex;

#[derive(Debug, Object)]
struct File {
    name: String,
    desc: Option<String>,
    content_type: Option<String>,
    filename: Option<String>,
    data: Vec<u8>,
}

#[derive(Debug, Object)]
struct FileInfo {
    name: String,
    desc: Option<String>,
    content_type: Option<String>,
    filename: Option<String>,
    size: u64,
}

#[derive(Debug, Response)]
enum GetFileResponse {
    #[oai(status = 200)]
    Ok(Json<FileInfo>),
    /// File not found
    #[oai(status = 404)]
    NotFound,
}

struct Status {
    id: u64,
    files: HashMap<u64, File>,
}

struct FileManager {
    status: Mutex<Status>,
}

#[derive(Debug, Multipart)]
struct UploadPayload {
    name: String,
    desc: Option<String>,
    file: Upload,
}

#[OpenApi]
impl FileManager {
    /// Upload file
    #[oai(path = "/files", method = "post")]
    async fn upload(&self, upload: UploadPayload) -> Result<Json<u64>> {
        let mut status = self.status.lock().await;
        let id = status.id;
        status.id += 1;

        let file = File {
            name: upload.name,
            desc: upload.desc,
            content_type: upload.file.content_type().map(ToString::to_string),
            filename: upload.file.file_name().map(ToString::to_string),
            data: upload.file.into_vec().await.map_err(Error::bad_request)?,
        };
        status.files.insert(id, file);
        Ok(Json(id))
    }

    /// Get file
    #[oai(path = "/files/:id", method = "get")]
    async fn get(&self, #[oai(name = "id", in = "path")] id: u64) -> GetFileResponse {
        let status = self.status.lock().await;
        match status.files.get(&id) {
            Some(file) => GetFileResponse::Ok(Json(FileInfo {
                name: file.name.clone(),
                desc: file.desc.clone(),
                content_type: file.content_type.clone(),
                filename: file.filename.clone(),
                size: file.data.len() as u64,
            })),
            None => GetFileResponse::NotFound,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:3000");
    poem::Server::new(listener)
        .await
        .unwrap()
        .run(
            OpenApiService::new(FileManager {
                status: Mutex::new(Status {
                    id: 1,
                    files: Default::default(),
                }),
            })
            .title("Upload Files")
            .ui_path("/ui"),
        )
        .await
        .unwrap();
}

use image::{ Pixel, Rgba,ImageFormat};
use imageproc::drawing::draw_hollow_rect_mut;
use imageproc::rect::Rect;
mod mtcnn;

use mtcnn::Mtcnn;
use std::collections::HashMap;

use poem::{listener::TcpListener, Error, Result};
use poem_openapi::{
    payload::{Binary, Json},
    types::multipart::Upload,
    Multipart, Object, OpenApi, OpenApiService, Response,
};
use tokio::sync::Mutex;

#[derive(Debug, Object, Clone)]
struct File {
    name: String,
    desc: Option<String>,
    content_type: Option<String>,
    filename: Option<String>,
    data: Vec<u8>,
}

#[derive(Debug, Response)]
enum GetFileResponse {
    #[oai(status = 200)]
    Ok(Binary, #[oai(header = "Content-Disposition")] String),
    /// File not found
    #[oai(status = 404)]
    NotFound,
}

struct Status {
    id: u64,
    files: HashMap<u64, File>,
}

#[derive(Debug, Multipart)]
struct UploadPayload {
    name: String,
    desc: Option<String>,
    file: Upload,
}

struct Api {
    status: Mutex<Status>,
}

#[OpenApi]
impl Api {
    /// Upload file
    #[oai(path = "/files", method = "post")]
    async fn upload(&self, upload: UploadPayload) -> Result<Json<u64>> {
        let mut status = self.status.lock().await;
        let id = status.id;
        status.id += 1;

        let mut file = File {
            name: upload.name,
            desc: upload.desc,
            content_type: upload.file.content_type().map(ToString::to_string),
            filename: upload.file.file_name().map(ToString::to_string),
            data: upload.file.into_vec().await.map_err(Error::bad_request)?,
        };
        let mtcnn = Mtcnn::new();
        let bytes = file.clone().data;
        let input_image = image::load_from_memory(&bytes).unwrap();
        let result = mtcnn.unwrap().run(&input_image);
        let _bbox = match result {
            Ok(bboxes) => {
                let mut output_image = input_image;
                let line = Rgba::from_slice(&[255u8, 0u8, 0u8, 255u8]);

                //Iterate through all bounding boxes
                for bbox in bboxes {
                    //Create a `Rect` from the bounding box.
                    let rect = Rect::at(bbox.x1 as i32, bbox.y1 as i32)
                        .of_size((bbox.x2 - bbox.x1) as u32, (bbox.y2 - bbox.y1) as u32);

                    //Draw a green line around the bounding box

                    draw_hollow_rect_mut(&mut output_image, rect, *line);
                }
                let memory_writer = &mut Vec::new();
                output_image.write_to(memory_writer, ImageFormat::Jpeg).unwrap();
                file.data=memory_writer.to_vec();
                // output_image.save("./111.jpg".to_string()).unwrap();
            }
            Err(e) => {
                println!("{:?}", e)
            }
        };

        status.files.insert(id, file);
        Ok(Json(id))
    }

    /// Get file
    #[oai(path = "/files/:id", method = "get")]
    async fn get(&self, #[oai(name = "id", in = "path")] id: u64) -> GetFileResponse {
        let status = self.status.lock().await;
        match status.files.get(&id) {
            Some(file) => {
                let mut content_disposition = String::from("attachment");
                if let Some(file_name) = &file.filename {
                    content_disposition += &format!("; filename={}", file_name);
                }
                GetFileResponse::Ok(file.data.clone().into(), content_disposition)
            }
            None => GetFileResponse::NotFound,
        }
    }
}

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:9729");
    poem::Server::new(listener)
        .await
        .unwrap()
        .run(
            OpenApiService::new(Api {
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

use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, Result};
use env_logger;
use futures::TryStreamExt;
use log::info;

#[actix_web::post("/upload_video")]
async fn upload_video(mut playload: Multipart) -> Result<HttpResponse, Error> {
    env_logger::init();
    while let Ok(Some(field)) = playload.try_next().await {
        let content_type = field.content_disposition();
        info!("Content-Type: {:?}", content_type);
    }

    Ok(HttpResponse::Ok().into())
}

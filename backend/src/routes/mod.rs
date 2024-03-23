use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, Result, web};
use futures::TryStreamExt;
use log::info;

async fn upload_video(mut payload: Multipart) -> Result<HttpResponse, Error> {
    env_logger::init();
    while let Ok(Some(field)) = payload.try_next().await {
        let content_type = field.content_disposition();
        info!("Content-Type: {:?}", content_type);
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upload_video").route(web::post().to(upload_video)));
}

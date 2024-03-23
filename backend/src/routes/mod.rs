use tokio::{fs::File, io::AsyncWriteExt};
use actix_multipart::Multipart;
use actix_web::{Error, HttpResponse, Result, web};
use futures::{StreamExt, TryStreamExt};

async fn upload_video(mut payload: Multipart) -> Result<HttpResponse, Error> {
    while let Ok(Some(mut field)) = payload.try_next().await {
        let content_disposition  = field.content_disposition();
        let filename = match content_disposition.get_filename() {
            Some(filename) => filename,
            None => return Err(actix_web::error::ErrorBadRequest("No filename present")),
        };

        let filepath = format!("./uploads/{}", sanitize_filename::sanitize(&filename));
        let mut f = File::create(filepath).await?;

        while let Some(chunk) = field.next().await {
            let data = chunk?;
            f.write_all(&data).await?;
        }
    }

    Ok(HttpResponse::Ok().finish())
}

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/upload_video").route(web::post().to(upload_video)));
}

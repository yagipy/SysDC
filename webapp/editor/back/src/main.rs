mod s3;

use actix_web::middleware::Logger;
use actix_web::http::StatusCode;
use actix_web::{get, App, HttpRequest, HttpResponse, HttpServer, Responder};

#[tokio::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();
    HttpServer::new(||
            App::new()
                .service(get_workspace_info)
                .service(get_workspace_files)
                .wrap(Logger::default())
        )
        .bind(("0.0.0.0", 50000))?
        .run()
        .await
}

#[get("/{workspace}")]
async fn get_workspace_info(req: HttpRequest) -> impl Responder {
    HttpResponse::build(StatusCode::OK)
        .content_type("application/json")
        .json(vec![req.path()])
}

#[get("/{workspace}/{file:.*}")]
async fn get_workspace_files(req: HttpRequest) -> impl Responder {
    match s3::get_file(req.uri().path()).await {
        Ok((mime, body)) =>
            HttpResponse::build(StatusCode::OK)
                .content_type(mime)
                .body(body),
        Err(_) =>
            HttpResponse::build(StatusCode::NOT_FOUND)
                .body("The specified file is not found.")
    }
}

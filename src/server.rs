use actix_web::{http::StatusCode, web, App, HttpResponse, HttpServer};

async fn index() -> Result<HttpResponse, actix_web::Error>{
    tracing::info!("Hi there!");
    Ok(
        HttpResponse::build(StatusCode::OK)
            .content_type("text/html")
            .body("Hi!")
            )
}

pub async fn run(config: crate::Config) -> std::io::Result<()> {
    let socket_addr = config.server.addr;

    tracing::info!("Server is starting at {}", socket_addr);
    HttpServer::new(move || {
        App::new().service(web::resource("/").route(web::get().to(index)))
    })
    .bind(socket_addr)?
    .run()
    .await
}
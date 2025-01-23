mod handler;


use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(handler::hello)
            .service(handler::echo)
            .route("/hey", web::get().to(handler::manual_hello))
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
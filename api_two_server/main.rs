//# actix-rt = "1.0"
//# actix-web = "2.0"
//# futures = "0.3"
//is a solution from: https://stackoverflow.com/questions/59642576/run-multiple-actix-app-on-different-ports
use actix_web::{web, App, HttpServer, Responder};
use futures::future;

async fn utils_one() -> impl Responder {
    "Utils one reached\n"
}

async fn health() -> impl Responder {
    "All good\n"
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let s1 = HttpServer::new(move || {
            App::new().service(web::scope("/utils").route("/one", web::get().to(utils_one)))
        })
        .bind("0.0.0.0:8081")?
        .run();
    let s2 = HttpServer::new(move || {
            App::new().service(web::resource("/health").route(web::get().to(health)))
        })
        .bind("0.0.0.0:8080")?
        .run();
    future::try_join(s1, s2).await?;

    Ok(())
}
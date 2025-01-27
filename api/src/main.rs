mod handler;
mod state;
use crate::{state::AppState, state::AppStateCounter};
use std::sync::Mutex;
use actix_web::{web, App, HttpServer, HttpResponse, guard};
use tokio::task;


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let address:&str ="127.0.0.1:8000";
    let counter = web::Data::new(AppStateCounter{
        counter: Mutex::new(0),
    });
    //let scope = web::scope("/users").service(handler::show_users);
    let server = HttpServer::new(move|| {
        App::new()
        
        .service(
            web::scope("")
                .guard(guard::Host("127.0.0.1:8000"))
                .route("/scope", web::to(|| async { HttpResponse::Ok().body("www") })),
        )
            .app_data(counter.clone())
            .app_data(web::Data::new(AppState {name: String::from("Actix Web"),}))
            .service(handler::counter)
            .service(handler::hello)
            .service(handler::echo)
            .route("/hey", web::get().to(handler::manual_hello))
            .service(handler::takestate)
            .service(handler::counter)
            .configure(handler::config)
            .service(web::scope("/api").configure(handler::scoped_config))
            .route("/",web::get().to(|| async { HttpResponse::Ok().body("/") }),)
            
    
            //.service(scope)
            
           
    })
    .bind(&address)?;
    let addr_clone = address.to_string();
    task::spawn(async move {
        println!("Server läuft und hört unter http://{}", addr_clone);
    });

    // Starte den Server
    server.run().await
}
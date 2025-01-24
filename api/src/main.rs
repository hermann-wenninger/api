mod handler;
mod state;
use crate::{state::AppState, state::AppStateCounter};
use std::sync::Mutex;


use actix_web::{web, App, HttpServer};


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let counter = web::Data::new(AppStateCounter{
        counter: Mutex::new(0),
    });
    //let scope = web::scope("/users").service(handler::show_users);
    HttpServer::new(move|| {
        App::new()
            .app_data(counter.clone())
            .app_data(web::Data::new(AppState {name: String::from("Actix Web"),}))
            .service(handler::counter)
            .service(handler::hello)
            .service(handler::echo)
            .route("/hey", web::get().to(handler::manual_hello))
            .service(handler::takestate)
            .service(handler::counter)
            //.service(scope)
            
           
    })
    .bind(("127.0.0.1", 8000))?
    .run()
    .await
}
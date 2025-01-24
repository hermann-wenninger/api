use crate::state;

#[allow(unused_imports)]
use actix_web::{get, post,  HttpResponse, Responder, web};

#[get("/counter")]
pub async fn counter(data: web::Data<state::AppStateCounter>) ->String{
    let mut counter = data.counter.lock().unwrap();
    *counter += 1;
    format!("Hallo, counter:{}",counter)
}

#[allow(unused_mut)]
#[get("/state")]
pub async fn takestate (data: web::Data<state::AppState>, x: web::Data<state::AppStateCounter>) -> String {
    let mut count = x.counter.lock().unwrap();
    format!("Hallo, state data:{}.....{}",data.name, count)
}

#[get("/")]
pub async fn hello() -> impl Responder {
    HttpResponse::Ok().body("Hello world!")
}

#[post("/echo")]
pub async fn echo(req_body: String) -> impl Responder {
    println!("req_body: {}", req_body);
    HttpResponse::Ok().body(req_body)
}

pub async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[allow(dead_code)]
pub async fn show_users() -> impl Responder {
    HttpResponse::Ok().body("Halla li all the users a,b,c,d...a1, a2.. x999999999")
}
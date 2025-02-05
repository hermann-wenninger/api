use actix_web::web;

mod path;

mod auth;

pub fn views_factory(app: &mut web::ServiceConfig) {

    auth::auth_factory(app);

}
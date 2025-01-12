use actix_web::web;
use crate::handlers::{create_user, get_user};

pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user").route(web::post().to(create_user)));
    cfg.service(web::resource("/user/{id}").route(web::get().to(get_user)));
}

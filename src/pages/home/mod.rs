use actix_web;

mod controller;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/").service(controller::page)
}

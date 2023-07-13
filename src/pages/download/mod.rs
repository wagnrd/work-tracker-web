use actix_web;

mod controller;

pub fn scope() -> actix_web::Scope {
    actix_web::web::scope("/download").service(controller::page)
}

use actix_web::get;
use sailfish::TemplateOnce;

use crate::utils::ssr;

#[derive(TemplateOnce)]
#[template(path = "pages/home/page.html")]
struct HomeTemplate {}

#[get("")]
pub async fn page(request: actix_web::HttpRequest) -> impl actix_web::Responder {
    let template = HomeTemplate {};
    ssr::render_page(template, request)
}

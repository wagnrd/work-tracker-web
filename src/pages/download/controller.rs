use actix_web::get;
use sailfish::TemplateOnce;

use crate::utils::ssr;

#[derive(TemplateOnce)]
#[template(path = "pages/download/page.html")]
struct DownloadTemplate {}

#[get("/")]
pub async fn page(request: actix_web::HttpRequest) -> impl actix_web::Responder {
    let template = DownloadTemplate {};
    ssr::render_page(template, request)
}

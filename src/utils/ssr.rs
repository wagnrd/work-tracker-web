use chrono::Datelike;
use sailfish::TemplateOnce;

#[derive(TemplateOnce)]
#[template(path = "main/page.html")]
struct MainPageTemplate {
    content: String,
    trademark_year: i32,
}

pub fn render_page<Template>(
    template: Template,
    request: actix_web::HttpRequest,
) -> impl actix_web::Responder
where
    Template: sailfish::TemplateOnce,
{
    // FIXME: if render_once() failed, en error page should be rendered
    let content = template.render_once().unwrap();

    let result = if request.headers().get("HX-Request").is_some() {
        content
    } else {
        MainPageTemplate {
            content,
            trademark_year: chrono::Local::now().year(),
        }
        .render_once()
        .unwrap()
    };

    actix_web::HttpResponse::Ok()
        .append_header(("Cache-Control", "max-age=3600, must-revalidate"))
        .body(result)
}

pub fn render<Template>(template: Template) -> impl actix_web::Responder
where
    Template: sailfish::TemplateOnce,
{
    let component = template.render_once().unwrap(); // FIXME: if render_once() failed, en error page should be rendered

    actix_web::HttpResponse::Ok()
        .append_header(("Cache-Control", "max-age=3600, must-revalidate"))
        .body(component)
}

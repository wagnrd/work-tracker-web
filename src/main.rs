mod pages;
mod utils;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    actix_web::HttpServer::new(|| {
        actix_web::App::new()
            .service(pages::home::scope())
            .service(pages::download::scope())
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

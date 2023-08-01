use actix_web::{get, web};
use actix_web_actors::ws;

struct AutoRefreshWS;

impl actix::Actor for AutoRefreshWS {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for AutoRefreshWS {
    fn handle(&mut self, _msg: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {}
}

#[get("/dev/autorefresh")]
pub async fn websocket(
    req: actix_web::HttpRequest,
    stream: web::Payload,
) -> impl actix_web::Responder {
    ws::start(AutoRefreshWS {}, &req, stream)
}

use actix_web::{get, web};
use actix_web_actors::ws;

struct AutoRefreshWS;

impl actix::Actor for AutoRefreshWS {
    type Context = ws::WebsocketContext<Self>;
}

/// Handler for ws::Message message
impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for AutoRefreshWS {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        if let Ok(ws::Message::Ping(msg)) = msg {
            ctx.pong(&msg)
        }
    }
}

#[get("/dev/autorefresh")]
pub async fn websocket(
    req: actix_web::HttpRequest,
    stream: web::Payload,
) -> impl actix_web::Responder {
    let resp = ws::start(AutoRefreshWS {}, &req, stream);
    println!("{:?}", resp);
    resp
}

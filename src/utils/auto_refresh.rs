use actix_web_actors::ws;

struct AutoRefreshWS;

impl actix::Actor for AutoRefreshWS {
    type Context = ws::WebsocketContext<Self>;
}

impl actix::StreamHandler<Result<ws::Message, ws::ProtocolError>> for AutoRefreshWS {
    fn handle(&mut self, _item: Result<ws::Message, ws::ProtocolError>, _ctx: &mut Self::Context) {}
}

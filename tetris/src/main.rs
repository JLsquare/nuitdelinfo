mod pieces;
mod tetris;

use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use crate::tetris::TetrisGame;

#[derive(Serialize, Deserialize)]
struct Move {
    direction: String,
}

struct TetrisWebSocket {
    game: TetrisGame,
}

impl Actor for TetrisWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.start_loop(ctx);
    }
}

impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for TetrisWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => self.handle_text(text.to_string(), ctx),
            _ => (),
        }
    }
}

impl TetrisWebSocket {
    fn new() -> Self {
        Self {
            game: TetrisGame::new(),
        }
    }

    fn handle_text(&mut self, text: String, ctx: &mut ws::WebsocketContext<Self>) {
        if text.contains("move") {
            let json: Move = serde_json::from_str(&text).unwrap();
            self.game.move_piece(json.direction.as_str());
            let message = serde_json::json!({
                "type": "matrix",
                "matrix": self.game.get_matrix(),
            });

            ctx.text(serde_json::to_string(&message).unwrap());
        }
    }

    fn start_loop(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(std::time::Duration::from_millis(500), |actor, ctx| {
            actor.game.update_loop();

            let matrix = actor.game.get_matrix();
            let message = serde_json::json!({
                "type": "matrix",
                "matrix": matrix,
            });

            ctx.text(serde_json::to_string(&message).unwrap());
        });
    }
}

async fn tetris_route(req: HttpRequest, stream: web::Payload) -> Result<HttpResponse, actix_web::Error> {
    ws::start(TetrisWebSocket::new(), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/ws/", web::get().to(tetris_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


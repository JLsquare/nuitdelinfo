mod pieces;
mod tetris;

use std::sync::RwLock;
use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};
use crate::tetris::TetrisGame;

#[derive(Serialize, Deserialize)]
struct Move {
    direction: String,
}

#[derive(Message, Clone, Serialize)]
#[rtype(result = "()")]
struct WsMessage {
    message: String,
}

struct AppState {
    sessions: RwLock<Vec<Addr<TetrisWebSocket>>>,
}

impl AppState {
    fn broadcast(&self, msg: WsMessage) {
        let sessions = self.sessions.read().unwrap();
        for session in sessions.iter() {
            session.do_send(msg.clone());
        }
    }

    fn add_session(&mut self, session: Addr<TetrisWebSocket>) {
        self.sessions.write().unwrap().push(session);
    }
}

struct TetrisWebSocket {
    game: TetrisGame,
    appstate: web::Data<RwLock<AppState>>,
    id: u32,
}

impl Actor for TetrisWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, ctx: &mut Self::Context) {
        self.appstate.write().unwrap().add_session(ctx.address());
        let message = serde_json::json!({
            "type": "id",
            "id": self.id,
        });
        ctx.text(serde_json::to_string(&message).unwrap());
        self.start_loop(ctx);
    }
}

impl Handler<WsMessage> for TetrisWebSocket {
    type Result = ();

    fn handle(&mut self, msg: WsMessage, ctx: &mut Self::Context) {
        ctx.text(msg.message);
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
    fn new(appstate: web::Data<RwLock<AppState>>, id: u32) -> Self {
        Self {
            game: TetrisGame::new(),
            appstate,
            id,
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
            if actor.game.is_game_over() {
                let message = serde_json::json!({
                    "type": "finished",
                    "score": actor.game.get_score(),
                });

                ctx.text(serde_json::to_string(&message).unwrap());

                ctx.stop();
            }

            let matrix = actor.game.get_matrix();
            let matrix_message = serde_json::json!({
                "type": "matrix",
                "matrix": matrix,
            });

            ctx.text(serde_json::to_string(&matrix_message).unwrap());

            let broadcast_matrix_message = serde_json::json!({
                "type": "broadcast_matrix",
                "id": actor.id,
                "matrix": matrix,
            });

            actor.appstate.write().unwrap().broadcast(WsMessage {
                message: serde_json::to_string(&broadcast_matrix_message).unwrap(),
            });

            let score_message = serde_json::json!({
                "type": "score",
                "score": actor.game.get_score(),
            });

            ctx.text(serde_json::to_string(&score_message).unwrap());
        });
    }
}

async fn tetris_route(
    req: HttpRequest,
    stream: web::Payload,
    data: web::Data<RwLock<AppState>>
) -> Result<HttpResponse, actix_web::Error> {
    let player_count = data.read().unwrap().sessions.read().unwrap().len() as u32;
    ws::start(TetrisWebSocket::new(data, player_count + 1), &req, stream)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let appstate = AppState {
        sessions: RwLock::new(Vec::new()),
    };
    let appstate = web::Data::new(RwLock::new(appstate));
    HttpServer::new(move || {
        App::new()
            .app_data(appstate.clone())
            .route("/ws/", web::get().to(tetris_route))
    })
        .bind("127.0.0.1:8080")?
        .run()
        .await
}


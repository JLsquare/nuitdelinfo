mod pieces;

use actix::{Actor, AsyncContext, StreamHandler};
use actix_web::{web, App, HttpRequest, HttpResponse, HttpServer};
use actix_web_actors::ws;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize)]
struct TetrisGame {
    matrix: Vec<Vec<u8>>,
    current_piece_position: Position,
    current_piece: pieces::Piece,
    current_rotation: u8,
}

impl TetrisGame {
    fn new() -> Self {
        Self {
            matrix: vec![vec![0; 10]; 20],
            current_piece_position: Position { x: 5, y: 0 },
            current_piece: pieces::Pieces::new().get_random_piece(),
            current_rotation: 0,
        }
    }

    fn move_piece(&mut self, direction: &str) {
        match direction {
            "ArrowLeft" => {
                self.current_piece_position.x -= 1;
                if !self.check_bounds() || self.check_collision() {
                    self.current_piece_position.x += 1;
                }
            }
            "ArrowRight" => {
                self.current_piece_position.x += 1;
                if !self.check_bounds() || self.check_collision() {
                    self.current_piece_position.x -= 1;
                }
            }
            "ArrowDown" => {
                self.current_piece_position.y += 1;
                if !self.check_bounds() || self.check_collision() {
                    self.current_piece_position.y -= 1;
                }
            }
            "r" => {
                self.current_rotation += 1;
                self.current_rotation %= 4;
            }
            _ => (),
        }
    }

    fn check_bounds(&self) -> bool {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 1 &&
                    (x + self.current_piece_position.x >= 10 ||
                    y + self.current_piece_position.y >= 20) {
                    return false;
                }
            }
        }

        true
    }

    fn check_collision(&self) -> bool {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 1 &&
                    self.matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] == 1 {
                    return true;
                }
            }
        }

        false
    }

    fn check_rows(&mut self) {
        let mut rows_to_remove = vec![];

        for (y, row) in self.matrix.iter().enumerate() {
            if row.iter().all(|&value| value == 1) {
                rows_to_remove.push(y);
            }
        }

        for row in rows_to_remove {
            self.matrix.remove(row);
            self.matrix.insert(0, vec![0; 10]);
        }
    }

    fn is_bottom_or_down_collision(&self) -> bool {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 1 {
                    if y + self.current_piece_position.y + 1 >= 20 {
                        return true;
                    }

                    if self.matrix[y + self.current_piece_position.y + 1][x + self.current_piece_position.x] == 1 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn apply_piece(&mut self) {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 1 {
                    self.matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] = 1;
                }
            }
        }
    }

    fn update_loop(&mut self) {
        if self.is_bottom_or_down_collision() {
            self.apply_piece();
            self.current_piece = pieces::Pieces::new().get_random_piece();
            self.current_piece_position = Position { x: 5, y: 0 };
            self.current_rotation = 0;
        } else {
            self.current_piece_position.y += 1;
        }

        self.check_rows();
    }

    fn get_matrix(&self) -> Vec<Vec<u8>> {
        let mut matrix = self.matrix.clone();
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value == 1 &&
                    x + self.current_piece_position.x < 10 &&
                    y + self.current_piece_position.y < 20 {
                    matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] = 1;
                }
            }
        }

        matrix
    }
}

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


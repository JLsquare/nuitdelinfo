use serde::{Deserialize, Serialize};
use crate::pieces;

#[derive(Serialize, Deserialize, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}

#[derive(Serialize, Deserialize)]
pub struct TetrisGame {
    matrix: Vec<Vec<u8>>,
    current_piece_position: Position,
    current_piece: pieces::Piece,
    current_rotation: u8,
    score: u32,
}

impl TetrisGame {
    pub fn new() -> Self {
        Self {
            matrix: vec![vec![0; 10]; 20],
            current_piece_position: Position { x: 4, y: 0 },
            current_piece: pieces::Pieces::new().get_random_piece(),
            current_rotation: 0,
            score: 0,
        }
    }

    pub fn is_game_over(&self) -> bool {
        self.matrix[0].iter().any(|&value| value != 0)
    }

    pub fn move_piece(&mut self, direction: &str) {
        match direction {
            "ArrowLeft" | "ArrowRight" | "ArrowDown" => self.move_direction(direction),
            "ArrowUp" => self.rotate_piece(),
            "r" => self.rotate_piece(),
            _ => (),
        }
    }

    fn move_direction(&mut self, direction: &str) {
        let (dx, dy) = match direction {
            "ArrowLeft" => (-1, 0),
            "ArrowRight" => (1, 0),
            "ArrowDown" => (0, 1),
            _ => (0, 0),
        };

        self.update_position(dx, dy);
    }

    fn update_position(&mut self, dx: isize, dy: isize) {
        let (new_x, new_y) = (
            (self.current_piece_position.x as isize + dx) as usize,
            (self.current_piece_position.y as isize + dy) as usize,
        );

        self.current_piece_position = Position { x: new_x, y: new_y };
        if !self.check_bounds() || self.check_collision() {
            self.current_piece_position = Position { x: new_x - dx as usize, y: new_y - dy as usize };
        }
    }

    fn rotate_piece(&mut self) {
        let original_rotation = self.current_rotation;
        let original_position = self.current_piece_position;

        self.current_rotation = (self.current_rotation + 1) % 4;

        if !self.check_bounds() || self.check_collision() {
            if !self.try_wall_kick(original_rotation, self.current_rotation) {
                self.adjust_piece_after_rotation();
            }
        }

        if !self.check_bounds() || self.check_collision() {
            self.current_piece_position = original_position;
            self.current_rotation = original_rotation;
        }
    }

    fn adjust_piece_after_rotation(&mut self) {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);
        let piece_width = rotated_piece_matrix[0].len();

        let shift_left = self.current_piece_position.x + piece_width - 10;
        let shift_right = 0 - self.current_piece_position.x as isize;

        if shift_left > 0 {
            self.current_piece_position.x -= shift_left;
        } else if shift_right > 0 {
            self.current_piece_position.x += shift_right as usize;
        }
    }

    fn is_bottom_or_down_collision(&self) -> bool {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value != 0 {
                    if y + self.current_piece_position.y + 1 >= 20 {
                        return true;
                    }

                    if self.matrix[y + self.current_piece_position.y + 1][x + self.current_piece_position.x] != 0 {
                        return true;
                    }
                }
            }
        }

        false
    }

    fn check_bounds(&self) -> bool {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value != 0 &&
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
                if value != 0 &&
                    self.matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] != 0 {
                    return true;
                }
            }
        }

        false
    }

    fn try_wall_kick(&mut self, initial_state: u8, final_state: u8) -> bool {
        let wall_kick_data = match (initial_state, final_state) {
            (0, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
            (1, 0) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
            (1, 2) => vec![(0, 0), (1, 0), (1, -1), (0, 2), (1, 2)],
            (2, 1) => vec![(0, 0), (-1, 0), (-1, 1), (0, -2), (-1, -2)],
            (2, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            (3, 2) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
            (3, 0) => vec![(0, 0), (-1, 0), (-1, -1), (0, 2), (-1, 2)],
            (0, 3) => vec![(0, 0), (1, 0), (1, 1), (0, -2), (1, -2)],
            _ => vec![]
        };

        for (dx, dy) in wall_kick_data {
            self.current_piece_position.x = (self.current_piece_position.x as isize + dx) as usize;
            self.current_piece_position.y = (self.current_piece_position.y as isize - dy) as usize; // Assuming positive y upwards

            if self.check_bounds() && !self.check_collision() {
                return true;
            }
        }

        false
    }

    fn check_rows(&mut self) {
        let mut rows_to_remove = vec![];

        for (y, row) in self.matrix.iter().enumerate() {
            if row.iter().all(|&value| value != 0) {
                rows_to_remove.push(y);
            }
        }

        match rows_to_remove.len() {
            1 => self.score += 100,
            2 => self.score += 300,
            3 => self.score += 500,
            4 => self.score += 800,
            _ => (),
        }

        for row in rows_to_remove {
            self.matrix.remove(row);
            self.matrix.insert(0, vec![0; 10]);
        }
    }

    fn apply_piece(&mut self) {
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value != 0 {
                    self.matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] = self.current_piece.color;
                }
            }
        }
    }

    pub fn update_loop(&mut self) {
        if self.is_bottom_or_down_collision() {
            self.apply_piece();
            self.current_piece = pieces::Pieces::new().get_random_piece();
            self.current_piece_position = Position { x: 4, y: 0 };
            self.current_rotation = 0;
        } else {
            self.current_piece_position.y += 1;
        }

        self.check_rows();
    }

    pub fn get_matrix(&self) -> Vec<Vec<u8>> {
        let mut matrix = self.matrix.clone();
        let rotated_piece_matrix = self.current_piece.get_rotated_matrix(self.current_rotation);

        for (y, row) in rotated_piece_matrix.iter().enumerate() {
            for (x, &value) in row.iter().enumerate() {
                if value != 0 &&
                    x + self.current_piece_position.x < 10 &&
                    y + self.current_piece_position.y < 20 {
                    matrix[y + self.current_piece_position.y][x + self.current_piece_position.x] = self.current_piece.color;
                }
            }
        }

        matrix
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }
}
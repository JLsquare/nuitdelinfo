use rand::Rng;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone)]
pub struct Piece {
    pub matrix: Vec<Vec<u8>>,
}

impl Piece {
    pub fn get_rotated_matrix(&self, rotations: u8) -> Vec<Vec<u8>> {
        rotate_matrix(self.matrix.clone(), rotations)
    }
}

pub struct Pieces {
    pieces: Vec<Piece>,
}

impl Pieces {
    pub fn new() -> Self {
        Self {
            pieces: vec![
                Piece {
                    matrix: vec![vec![1, 1], vec![1, 1]],
                },
                Piece {
                    matrix: vec![vec![1, 1, 1, 1]],
                },
                Piece {
                    matrix: vec![vec![1, 1, 0], vec![0, 1, 1]],
                },
                Piece {
                    matrix: vec![vec![0, 1, 1], vec![1, 1, 0]],
                },
                Piece {
                    matrix: vec![vec![1, 0, 0], vec![1, 1, 1]],
                },
                Piece {
                    matrix: vec![vec![0, 0, 1], vec![1, 1, 1]],
                },
                Piece {
                    matrix: vec![vec![1, 1, 1], vec![0, 0, 1]],
                },
            ],
        }
    }

    pub fn get_random_piece(&self) -> Piece {
        let mut rng = rand::thread_rng();
        let random_index = rng.gen_range(0..self.pieces.len());
        self.pieces[random_index].clone()
    }
}

fn rotate_matrix(matrix: Vec<Vec<u8>>, rotations: u8) -> Vec<Vec<u8>> {
    let mut rotated_matrix = matrix.clone();

    for _ in 0..rotations % 4 {
        rotated_matrix = rotate_90_clockwise(rotated_matrix);
    }

    rotated_matrix
}

fn rotate_90_clockwise(matrix: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let rows = matrix.len();
    let cols = matrix.first().map_or(0, Vec::len);
    let mut new_matrix = vec![vec![0; rows]; cols];

    for y in 0..rows {
        for x in 0..cols {
            new_matrix[x][rows - 1 - y] = matrix[y][x];
        }
    }

    new_matrix
}
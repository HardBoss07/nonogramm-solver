use std::fs;
use crate::puzzle::{PuzzleJson, PuzzleData};

pub struct Nonogramm {
    pub size: usize,
    pub rows: Vec<Vec<u8>>,
    pub cols: Vec<Vec<u8>>,
    pub solution: String,
    pub board: Vec<char>,
}

impl Nonogramm {
    pub fn load(puzzle_id: &str) -> Self {
        let data = fs::read_to_string("puzzles.json")
            .expect("Failed to read puzzles.json");

        let puzzle_data: PuzzleData =
            serde_json::from_str(&data).expect("Failed to parse JSON");

        let puzzle = puzzle_data.puzzles
            .into_iter()
            .find(|p| p.id == puzzle_id)
            .expect("Puzzle not found");

        Nonogramm {
            size: puzzle.size,
            rows: puzzle.rows,
            cols: puzzle.cols,
            solution: puzzle.solution,
            board: vec!['?'; puzzle.size * puzzle.size],
        }
    }

    pub fn to_string_board(&self) -> String {
        let mut result = String::new();

        for i in 0..self.size {
            for j in 0..self.size {
                let idx = i * self.size + j;
                let c = self.board.get(idx).copied().unwrap_or('?');

                result.push(c);
            }
            result.push('\n');
        }
        result
    }

}
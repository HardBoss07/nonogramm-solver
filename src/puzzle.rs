use serde::Deserialize;

#[derive(Deserialize)]
pub struct PuzzleData {
    pub puzzles: Vec<PuzzleJson>,
}

#[derive(Deserialize)]
pub struct PuzzleJson {
    pub id: String,
    pub name: String,
    pub size: usize,
    pub rows: Vec<Vec<u8>>,
    pub cols: Vec<Vec<u8>>,
    pub solution: String,
}

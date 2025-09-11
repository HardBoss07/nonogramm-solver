mod nonogramm;
mod puzzle;

use nonogramm::Nonogramm;

fn main() {

    let puzzle = Nonogramm::load("puzzle1");
    println!("Rows: {:?}\nCols: {:?}\nSize: {}", puzzle.rows, puzzle.cols, puzzle.size);
    println!("{}", puzzle.to_string_board());
}

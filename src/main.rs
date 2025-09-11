mod nonogramm;
mod puzzle;
mod solver;

use nonogramm::Nonogramm;
use solver::Solver;

fn main() {
    let puzzle = Nonogramm::load("puzzle1");
    let mut solver = Solver::new(puzzle.clone());

    println!("{}", puzzle.to_string_board());

    solver.nonogramm.board = solver.solve();
    println!("{}", Nonogramm::to_string_board(&solver.nonogramm));
}

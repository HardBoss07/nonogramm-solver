mod nonogramm;
mod puzzle;
mod solver;

use nonogramm::Nonogramm;
use solver::Solver;

fn main() {
    let puzzle = Nonogramm::load("puzzle3");
    let mut solver = Solver::new(puzzle.clone());

    println!("{}", puzzle.to_string_board());

    solver.solve();
    println!("{}", Nonogramm::to_string_board(&solver.nonogramm));
}

use crate::Nonogramm;

pub struct Solver {
    pub nonogramm: Nonogramm,
}

impl Solver {
    pub fn new(n: Nonogramm) -> Self {
        Self {
            nonogramm: n,
        }
    }

    pub fn solve(&mut self) -> Vec<char> {
        let mut board = self.nonogramm.board.clone();

        let size = self.nonogramm.size;

        // rows
        for (row_idx, clues) in self.nonogramm.rows.iter().enumerate() {
            if self.is_golden_rule(clues) {
                let mut idx = row_idx * size;
                for (i, &clue) in clues.iter().enumerate() {
                    for _ in 0..clue {
                        board[idx] = 'X';
                        idx += 1;
                    }

                    if i != clues.len() - 1 {
                        board[idx] = '.';
                        idx += 1;
                    }
                }
            }
        }

        // cols
        for (col_idx, clues) in self.nonogramm.cols.iter().enumerate() {
            if self.is_golden_rule(clues) {
                let mut row = 0;
                for (i, &clue) in clues.iter().enumerate() {
                    for _ in 0..clue {
                        let idx = row * size + col_idx;
                        board[idx] = 'X';
                        row += 1;
                    }
                    if i != clues.len() - 1 {
                        let idx = row * size + col_idx;
                        board[idx] = '.';
                        row += 1;
                    }
                }
            }
        }

        board.clone()
    }

    fn is_golden_rule(&self, clues: &[u8]) -> bool {
        clues.iter().map(|&x| x as usize).sum::<usize>() + (clues.len() - 1) == self.nonogramm.size
    }
}
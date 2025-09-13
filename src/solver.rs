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

    pub fn solve(&mut self) {
        let mut board = self.nonogramm.board.clone();

        let size = self.nonogramm.size;

        // rows
        for (row_index, clues) in self.nonogramm.rows.iter().enumerate() {
            if self.is_golden_rule(clues) {
                let mut index = row_index * size;
                for (i, &clue) in clues.iter().enumerate() {
                    for _ in 0..clue {
                        board[index] = 'X';
                        index += 1;
                    }

                    if i != clues.len() - 1 {
                        board[index] = '.';
                        index += 1;
                    }
                }
            }
        }

        // cols
        for (col_index, clues) in self.nonogramm.cols.iter().enumerate() {
            if self.is_golden_rule(clues) {
                let mut row = 0;
                for (i, &clue) in clues.iter().enumerate() {
                    for _ in 0..clue {
                        let index = row * size + col_index;
                        board[index] = 'X';
                        row += 1;
                    }
                    if i != clues.len() - 1 {
                        let index = row * size + col_index;
                        board[index] = '.';
                        row += 1;
                    }
                }
            }
        }

        // next: if only 1 clue and clue is bigger or equal to size / 2 fill in fixed places

        self.nonogramm.board = board;
    }

    fn is_golden_rule(&self, clues: &[u8]) -> bool {
        clues.iter().map(|&x| x as usize).sum::<usize>() + (clues.len() - 1) == self.nonogramm.size
    }
}
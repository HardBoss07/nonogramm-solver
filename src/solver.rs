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

    /*fn is_row_solved(&self, clues: &[u8]) -> bool {
        let mut filled_clue_len = 0;

        for (i, &clue) in clues.iter().enumerate() {
            match self.nonogramm.board.get(self.coords_to_index(i, 0)) {
                Some(&'X') => {
                    filled_clue_len += 1;
                    if filled_clue_len > clue as usize {
                        return false;
                    } else if filled_clue_len == clue as usize {
                        filled_clue_len = 0;
                    }
                }
                Some(&'.') => return false,
                Some(&'?') => return false,
                _ => return false,
            }
        }


    }*/

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * self.nonogramm.size + col
    }
}
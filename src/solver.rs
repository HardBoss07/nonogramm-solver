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

        if self.is_solved() {
            println!("Puzzle solved");
        }

        self.nonogramm.board = board;
    }

    fn is_golden_rule(&self, clues: &[u8]) -> bool {
        clues.iter().map(|&x| x as usize).sum::<usize>() + (clues.len() - 1) == self.nonogramm.size
    }

    pub fn is_solved(&self) -> bool {
        let size = self.nonogramm.size;

        // check all rows
        for (row, clues) in self.nonogramm.rows.iter().enumerate() {
            if !self.is_row_solved(row, clues) {
                return false;
            }
        }

        // check all cols
        for (col, clues) in self.nonogramm.cols.iter().enumerate() {
            if !self.is_col_solved(col, clues) {
                return false;
            }
        }

        true
    }

    fn coords_to_index(&self, row: usize, col: usize) -> usize {
        row * self.nonogramm.size + col
    }

    fn is_row_solved(&self, row: usize, clues: &[u8]) -> bool {
        let size = self.nonogramm.size;
        self.is_line_solved(size, clues, |col| {
            self.nonogramm.board.get(self.coords_to_index(row, col)).copied()
        })
    }

    fn is_col_solved(&self, col: usize, clues: &[u8]) -> bool {
        let size = self.nonogramm.size;
        self.is_line_solved(size, clues, |row| {
            self.nonogramm.board.get(self.coords_to_index(row, col)).copied()
        })
    }

    fn is_line_solved<F>(&self, len: usize, clues: &[u8], mut get: F) -> bool
    where
        F: FnMut(usize) -> Option<char>,
    {
        let mut clue_index = 0;
        let mut filled_clue_len = 0;
    
        for i in 0..len {
            match get(i) {
                Some('X') => {
                    filled_clue_len += 1;
                    if clue_index >= clues.len() || filled_clue_len > clues[clue_index] as usize {
                        return false;
                    }
                }
                Some('.') => {
                    if filled_clue_len > 0 {
                        if filled_clue_len != clues[clue_index] as usize {
                            return false;
                        }
                        clue_index += 1;
                        filled_clue_len = 0;
                    }
                }
                Some('?') => return false,
                _ => return false,
            }
        }
    
        // end of line: must close last group if there was one
        if filled_clue_len > 0 {
            if clue_index >= clues.len() || filled_clue_len != clues[clue_index] as usize {
                return false;
            }
            clue_index += 1;
        }
    
        clue_index == clues.len()
    }
}
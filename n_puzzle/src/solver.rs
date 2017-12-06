use std::{error, fmt};

#[derive(Debug, Copy, Clone)]
pub enum Error {
    UnmatchingSizes,
    Unsolvable,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            UnmatchingSizes => "sizes doesn't match",
            Unsolvable => "puzzle is unsolvable",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
    }
}

pub struct Board {
    pub data: Box<[u32]>,
    pub line_size: usize,
}

impl Board {
    pub fn new(data: Box<[u32]>, line_size: usize) -> Self {
        Self { data, line_size }
    }

    pub fn inversions(&self) -> usize {
        let mut inversions = 0;
        for (i, &current) in self.data.iter().enumerate() {
            for &x in self.data.iter().skip(i + 1) {
                if x < current && current != 0 && x != 0 {
                    inversions += 1;
                }
            }
        }
        inversions
    }
}

pub struct Solver {
    board: Board,
    expected: Board,
}

fn is_solvable(board: &Board, expected: &Board) -> bool {
    let mut board_inv = board.inversions();
    let mut expected_inv = expected.inversions();

    if board.data.len() % 2 == 0 {
        board_inv += board.data.iter().position(|x| *x == 0).unwrap() / board.data.len();
        expected_inv += expected.data.iter().position(|x| *x == 0).unwrap() / board.data.len();
    }

    board_inv % 2 == expected_inv % 2
}


impl Solver {
    pub fn new(board: Board, expected: Board) -> Result<Self, Error> {
        if board.data.len() != expected.data.len() {
            Err(Error::UnmatchingSizes)
        }
        else if !is_solvable(&board, &expected) {
            Err(Error::Unsolvable)
        } else {
            Ok(Self { board, expected })
        }
    }

}
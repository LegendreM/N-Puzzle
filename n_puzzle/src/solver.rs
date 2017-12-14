extern crate panoradix;

use panoradix::RadixMap;

use std::{error, fmt, rc};


type Teal = u32;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
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

#[derive(Debug, Clone)]
pub struct Board {
    pub data: Box<[Teal]>,
    pub line_size: usize,
}

impl Board {
    pub fn new(data: Box<[Teal]>, line_size: usize) -> Self {
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    pub cost: usize,
    pub board: Board,
    pub parent: Option<rc::Rc<State>>,
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        self.cost.cmp(&other.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
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

    pub fn solve(&self) {
        let mut open_heap = BinaryHeap::new();
        let mut close_map: RadixMap<Vec<Teal>, usize> = RadixMap::new();

        open_heap.push(State { cost: 0, board: self.board.clone(), None});

        unimplemented!();
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn unmatching_sizes() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 0].into_boxed_slice(), 4);

        let solver_result = Solver::new(board, expected);

        assert!(solver_result.is_err());

        assert_eq!(solver_result.unwrap_err(), Error::UnmatchingSizes);
    }

    #[test]
    fn unsolvable_simple() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 4, 5, 6, 8, 7, 0].into_boxed_slice(), 3);

        let solver_result = Solver::new(board, expected);

        assert!(solver_result.is_err());

        assert_eq!(solver_result.unwrap_err(), Error::Unsolvable);
    }

    #[test]
    fn unsolvable_medium() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![2, 1, 3, 5, 4, 6, 8, 7, 0].into_boxed_slice(), 3);

        let solver_result = Solver::new(board, expected);

        assert!(solver_result.is_err());

        assert_eq!(solver_result.unwrap_err(), Error::Unsolvable);
    }

    #[test]
    fn unsolvable_hard() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![2, 1, 5, 4, 3, 6, 8, 7, 0].into_boxed_slice(), 3);

        let solver_result = Solver::new(board, expected);

        assert!(solver_result.is_err());

        assert_eq!(solver_result.unwrap_err(), Error::Unsolvable);
    }

    #[test]
    fn solvable() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 4, 5, 6, 0, 7, 8].into_boxed_slice(), 3);

        let solver_result = Solver::new(board, expected);

        assert!(solver_result.is_ok());
    }
}
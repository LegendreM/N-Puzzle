use panoradix::RadixMap;

use std::{error, fmt};
use std::collections::BinaryHeap;
use std::cmp::Ordering;

type Tile = u32;

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

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub data: Box<[Tile]>,
    pub line_size: usize,
}

impl Board {
    pub fn new(data: Box<[Tile]>, line_size: usize) -> Self {
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

    pub fn children(&self) -> Vec<Self> {
        let mut children = Vec::with_capacity(4);
        let zero = self.data.iter().position(|&x| x == 0).unwrap();
        let line_size = self.line_size;

        if zero > line_size {
            let mut board = self.clone();
            board.data.swap(zero, zero - line_size);
            children.push(board);
        }
        if zero < line_size * (line_size - 1) {
            let mut board = self.clone();
            board.data.swap(zero, zero + line_size);
            children.push(board);
        }
        if zero % line_size > 0 {
            let mut board = self.clone();
            board.data.swap(zero, zero - 1);
            children.push(board);
        }
        if zero % line_size < line_size - 1 {
            let mut board = self.clone();
            board.data.swap(zero, zero + 1);
            children.push(board);
        }

        children
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State<'a> {
    pub cost: usize,
    pub board: Board,
    pub parent: Option<&'a State<'a>>,
}

impl<'a> State<'a> {
    pub fn children(&'a self) -> Vec<State<'a>> {
        self.board.children().into_iter().map(|board| Self {
            cost: self.cost + 1,
            board: board,
            parent: Some(self)
        }).collect()
    }
}

impl<'a> Ord for State<'a> {
    fn cmp(&self, other: &State) -> Ordering {
        // Notice that the we flip the ordering on costs.
        other.cost.cmp(&self.cost)
    }
}

// `PartialOrd` needs to be implemented as well.
impl<'a> PartialOrd for State<'a> {
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

    pub fn solve(&self) -> Vec<Board> {
        let mut open_heap = BinaryHeap::new();
        let mut close_map: RadixMap<[Tile], usize> = RadixMap::new();

        open_heap.push(State{ cost: 0, board: self.board.clone(), parent: None });

        loop {
            let state = open_heap.pop().expect("invalid empty open heap");
            if state.board == self.expected {
                unimplemented!("I found the answer !");
            }
        }
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
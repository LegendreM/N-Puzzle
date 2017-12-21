use std::collections::HashSet;

use std::{error, fmt};
use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::rc::Rc;
use board::{Board, Tile};
use heuristic::Heuristic;

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

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,    
}

impl Move {
    pub fn new(parent: &Board, board: &Board) -> Self {
        let line_size = board.line_size;
        let zero = board.data.iter().position(|&x| x == 0).unwrap();
        let parent_zero = parent.data.iter().position(|&x| x == 0).unwrap();
        if zero == parent_zero + line_size {
            Move::Up
        } else if zero + line_size == parent_zero {
            Move::Down
        } else if zero + 1 == parent_zero {
            Move::Right
        } else /*zero == parent_zero + 1*/ {
            Move::Left
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    pub cost: usize,
    pub board: Board,
    pub parent: Option<Rc<State>>,
}

impl State {
    pub fn children<H: Heuristic>(&self, expected: &Board, heuristic: &H) -> Vec<State> {
        let parent = Rc::new(self.clone());
        self.board.children().into_iter().map(|board| Self {
            cost: self.cost + heuristic.distance(&board) + 1,
            board: board,
            parent: Some(parent.clone())
        }).collect()
    }

    pub fn build_path(&self) -> Vec<Move> {
        fn precedent_move(state: &State, path: &mut Vec<Move>) {
            if let Some(ref parent) = state.parent {
                precedent_move(parent, path);
                let move_ = Move::new(&parent.board, &state.board);
                path.push(move_);
            }
        }
        let mut path = Vec::new();
        precedent_move(self, &mut path);
        path
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        // Notice that the we flip the ordering on costs.
        Some(other.cost.cmp(&self.cost))
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

    if board.line_size % 2 == 0 {
        board_inv += board.data.iter().position(|x| *x == 0).unwrap() / board.line_size;
        expected_inv += expected.data.iter().position(|x| *x == 0).unwrap() / board.line_size;
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

    pub fn solve<H: Heuristic>(&self) -> Vec<Move> {
        let heuristic = H::new(&self.expected);
        let mut open_heap = BinaryHeap::new();
        let mut close_map = HashSet::new();

        open_heap.push(State{ cost: 0, board: self.board.clone(), parent: None });

        loop {
            let state = open_heap.pop().expect("invalid empty open heap");
            if state.board.data == self.expected.data {
                return state.build_path();
            }
            let children = state.children(&self.expected, &heuristic);
            for child in children {
                if !close_map.contains(&child.board.data) {
                    open_heap.push(child);
                }
            }
            close_map.insert(state.board.data.clone());
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use heuristic::Manhattan;
    use heuristic::Dijkstra;

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

    #[test]
    fn state_tree() {
        let board = Board::new(vec![1, 2, 3, 4, 0, 6, 7, 8, 5].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 4, 5, 6, 0, 7, 8].into_boxed_slice(), 3);
        let dijkstra = Dijkstra::new(&expected);

        let mut open_heap = BinaryHeap::new();

        let parent = State { cost: 0, board: board, parent: None };

        let children = parent.children(&expected, &dijkstra);
        {
            {
                for child in children {
                    open_heap.push(child);
                }
            }
            let parent = open_heap.pop().unwrap();
            let children = parent.children(&expected, &dijkstra);
            {
                for child in children {
                    open_heap.push(child);
                }
            }
            open_heap.pop();
        }
    }

    #[test]
    fn solver_3x3_manhattan() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 0, 4, 6, 7, 5, 8].into_boxed_slice(), 3);
        
        let solver = Solver::new(board, expected).unwrap();
        let result = solver.solve::<Manhattan>();

        let expected_result = &[Move::Right, Move::Down, Move::Right];
        assert_eq!(&result, expected_result)
    }

    #[test]
    fn solver_3x3_dijkstra() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 0, 4, 6, 7, 5, 8].into_boxed_slice(), 3);
        
        let solver = Solver::new(board, expected).unwrap();
        let result = solver.solve::<Dijkstra>();

        let expected_result = &[Move::Right, Move::Down, Move::Right];
        assert_eq!(&result, expected_result)
    }
}
use std::collections::{HashMap, BinaryHeap};
use std::{error, fmt};
use board::Board;
use state::State;
use tile_move::Move;
use heuristic::Heuristic;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Error {
    UnmatchingSizes,
    Unsolvable,
}

impl error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::UnmatchingSizes => "sizes doesn't match",
            Error::Unsolvable => "puzzle is unsolvable",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str(error::Error::description(self))
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

    pub fn solve<H: Heuristic>(&self) -> (usize, usize, Vec<Move>) {
        let heuristic = H::new(&self.expected);
        let mut open_heap = BinaryHeap::new();
        let mut close_map = HashMap::new();
        let mut time_complexity = 0;
        let mut mem_complexity = 1;
        let mut mem_complexity_max = 0;

        // will be poped just after
        open_heap.push(State{ cost: 0, distance: 0, board: self.board.clone(), parent: None });

        loop {
            let state = open_heap.pop().expect("invalid empty open heap");
            mem_complexity -= 1;
            if state.board.data == self.expected.data {
                return (mem_complexity_max, time_complexity, state.build_path());
            }
            let children = state.children(&heuristic);
            for child in children {
                if close_map.get(&child.board.data).map_or(true, |&cost| cost > child.cost) {
                    open_heap.push(child);
                    time_complexity += 1;
                    mem_complexity += 1;
                }
            }
            if mem_complexity > mem_complexity_max {
                mem_complexity_max = mem_complexity;
            }
            close_map.insert(state.board.data.clone(), state.cost);
        }
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use heuristic::Manhattan;
    use heuristic::Dijkstra;
    use heuristic::MissPlaced;
    use heuristic::Euclidean;

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

        let parent = State { cost: 0, distance: 0, board: board, parent: None };

        let children = parent.children(&dijkstra);
        {
            {
                for child in children {
                    open_heap.push(child);
                }
            }
            let parent = open_heap.pop().unwrap();
            let children = parent.children(&dijkstra);
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

    #[test]
    fn solver_3x3_euclidean() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 0, 4, 6, 7, 5, 8].into_boxed_slice(), 3);
        
        let solver = Solver::new(board, expected).unwrap();
        let result = solver.solve::<Euclidean>();

        let expected_result = &[Move::Right, Move::Down, Move::Right];
        assert_eq!(&result, expected_result)
    }

    #[test]
    fn solver_3x3_miss_placed() {
        let board = Board::new(vec![1, 2, 3, 4, 5, 6, 7, 8, 0].into_boxed_slice(), 3);
        let expected = Board::new(vec![1, 2, 3, 0, 4, 6, 7, 5, 8].into_boxed_slice(), 3);
        
        let solver = Solver::new(board, expected).unwrap();
        let result = solver.solve::<MissPlaced>();

        let expected_result = &[Move::Right, Move::Down, Move::Right];
        assert_eq!(&result, expected_result)
    }
}
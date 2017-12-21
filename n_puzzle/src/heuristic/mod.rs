mod manhattan;

pub use self::manhattan::Manhattan;
use board::Board;

pub trait Heuristic {
    fn distance(current: &Board, expected: &Board) -> usize;
}
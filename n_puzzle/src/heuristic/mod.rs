mod manhattan;
mod dijkstra;

pub use self::manhattan::Manhattan;
pub use self::dijkstra::Dijkstra;
use board::Board;

pub trait Heuristic {
    fn distance(current: &Board, expected: &Board) -> usize;
}
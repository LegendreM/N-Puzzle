use heuristic::Heuristic;
use board::Board;

pub struct Dijkstra;

impl Heuristic for Dijkstra {
    fn distance(_current: &Board, _expected: &Board) -> usize {
        0
    }
}
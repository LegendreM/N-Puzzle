use heuristic::Heuristic;
use board::Board;

pub struct Dijkstra;

impl Heuristic for Dijkstra {
    fn new(_expected: &Board) -> Self {
        Dijkstra
    }

    fn distance(&self, _current: &Board) -> usize {
        0
    }
}
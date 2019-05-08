use crate::heuristic::{Heuristic, index_positions};
use crate::board::Board;

pub struct MissPlaced {
    positions: Box<[(isize, isize)]>,
}

impl Heuristic for MissPlaced {
    fn new(expected: &Board) -> Self {
        Self{ positions: index_positions(expected) }
    }

    fn distance(&self, current: &Board) -> usize {
        let line_size = current.line_size as isize;
        let mut cost = 0;
        for (i, &tile) in current.data.iter().enumerate() {
            let i = i as isize;
            let (exp_x, exp_y) = self.positions[tile as usize];
            if i - exp_x - exp_y * line_size != 0 {
                cost += 1;
            }
        }
        cost
    }
}
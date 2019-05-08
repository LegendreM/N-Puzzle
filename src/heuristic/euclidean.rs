use crate::heuristic::{Heuristic, index_positions};
use crate::board::Board;

pub struct Euclidean {
    positions: Box<[(isize, isize)]>,
}

impl Heuristic for Euclidean {
    fn new(expected: &Board) -> Self {
        Self{ positions: index_positions(expected) }
    }

    fn distance(&self, current: &Board) -> usize {
        let line_size = current.line_size as isize;
        let mut cost = 0;
        for (i, &tile) in current.data.iter().enumerate() {
            let i = i as isize;
            let (exp_x, exp_y) = self.positions[tile as usize];
            let (cur_x, cur_y) = (i % line_size, i / line_size);
            let x_sum = exp_x - cur_x;
            let y_sum = exp_y - cur_y;
            cost += ((x_sum * x_sum + y_sum * y_sum) as f64).sqrt() as usize;
        }
        cost
    }
}
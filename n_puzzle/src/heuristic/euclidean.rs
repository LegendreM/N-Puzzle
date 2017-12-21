use heuristic::{Heuristic, index_positions};
use board::Board;

pub struct Euclidean {
    positions: Box<[(isize, isize)]>,
}

impl Heuristic for Euclidean {
    fn new(expected: &Board) -> Self {
        Self{ positions: index_positions(expected) }
    }

    fn distance(&self, current: &Board) -> usize {
        // let line_size = current.line_size;
        // let mut cost = 0;
        // for (i, &tile) in current.data.iter().enumerate() {
        //     let expected_pos = expected.data.iter().position(|&x| x == tile).unwrap();
        //     let linear_dist = (i as isize - expected_pos as isize).abs() as usize;
        //     cost += (linear_dist % line_size) + (linear_dist / line_size);
        // }
        // cost
        unimplemented!()
    }
}
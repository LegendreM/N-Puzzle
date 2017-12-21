use heuristic::Heuristic;
use board::Board;

pub struct Manhattan;

impl Heuristic for Manhattan {
    fn distance(current: &Board, expected: &Board) -> usize {
        let line_size = current.line_size;
        let mut cost = 0;
        for (i, &tile) in current.data.iter().enumerate() {
            let expected_pos = expected.data.iter().position(|&x| x == tile).unwrap();
            let linear_dist = (i as isize - expected_pos as isize).abs() as usize;
            cost += (linear_dist % line_size) + (linear_dist / line_size);
        }
        cost
    }
}
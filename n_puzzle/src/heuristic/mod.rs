mod manhattan;
mod dijkstra;

pub use self::manhattan::Manhattan;
pub use self::dijkstra::Dijkstra;
use board::Board;

fn index_positions(board: &Board) -> Box<[(isize, isize)]> {
        let line_size = board.line_size;
        let mut positions = vec![(0, 0); board.data.len()];

        for (i, &p) in board.data.iter().enumerate() {
            positions[p as usize] = ((i % line_size) as isize, (i / line_size) as isize);
        }

        positions.into_boxed_slice()
}

pub trait Heuristic {
    fn new(expected: &Board) -> Self;
    fn distance(&self, current: &Board) -> usize;
}
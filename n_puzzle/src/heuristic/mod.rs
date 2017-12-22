mod manhattan;
mod dijkstra;
mod euclidean;
mod miss_placed;
mod out_of_raw;

pub use self::manhattan::Manhattan;
pub use self::dijkstra::Dijkstra;
pub use self::euclidean::Euclidean;
pub use self::miss_placed::MissPlaced;
pub use self::out_of_raw::OutOfRaw;
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
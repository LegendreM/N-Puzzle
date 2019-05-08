use crate::board::Board;

#[derive (Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum Move {
    Up,
    Down,
    Left,
    Right,    
}

impl Move {
    pub fn new(parent: &Board, board: &Board) -> Self {
        let line_size = board.line_size;
        let zero = board.data.iter().position(|&x| x == 0).unwrap();
        let parent_zero = parent.data.iter().position(|&x| x == 0).unwrap();

        if zero == parent_zero + line_size {
            Move::Up
        } else if zero + line_size == parent_zero {
            Move::Down
        } else if zero + 1 == parent_zero {
            Move::Right
        } else /* zero == parent_zero + 1 */ {
            Move::Left
        }
    }
}
use std::hash::{Hash, Hasher};

pub type Tile = u8;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Board {
    pub data: Box<[Tile]>,
    pub line_size: usize,
}

impl Board {
    pub fn new(data: Box<[Tile]>, line_size: usize) -> Self {
        Self { data, line_size }
    }

    pub fn inversions(&self) -> usize {
        let mut inversions = 0;
        for (i, &current) in (&self.data[..self.data.len() - 1]).iter().enumerate() {
            for &x in self.data.iter().skip(i + 1) {
                if x < current && current != 0 && x != 0 {
                    inversions += 1;
                }
            }
        }
        inversions
    }

    pub fn children(&self) -> Vec<Self> {
        let mut children = Vec::with_capacity(4);
        let zero = self.data.iter().position(|&x| x == 0).unwrap();
        let line_size = self.line_size;

        if zero > line_size {
            let mut board = self.clone();
            board.data.swap(zero, zero - line_size);
            children.push(board);
        }
        if zero < line_size * (line_size - 1) {
            let mut board = self.clone();
            board.data.swap(zero, zero + line_size);
            children.push(board);
        }
        if zero % line_size > 0 {
            let mut board = self.clone();
            board.data.swap(zero, zero - 1);
            children.push(board);
        }
        if zero % line_size < line_size - 1 {
            let mut board = self.clone();
            board.data.swap(zero, zero + 1);
            children.push(board);
        }

        children
    }
}

impl Hash for Board {
    fn hash<H: Hasher>(&self, state: &mut H) {
        let mut hash: usize = 0;

        for (i, &tile) in self.data.iter().enumerate() {
            hash ^= (tile as usize & 0xF) << ((i & 7) << 2);
        }

        state.write_usize(hash);
    }
}
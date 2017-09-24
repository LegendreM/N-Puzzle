#[derive(PartialEq)]
pub enum BoardType {
    Linear,
    Snail,
}

pub struct Board {
    data: Box<[u32]>,
    line_size: usize,
    board_type: BoardType,
}

impl Board {
    pub fn new(data: Box<[u32]>, line_size: usize, board_type: BoardType) -> Board {
        Board {
            data: data,
            line_size: line_size,
            board_type: board_type,
        }
    }

    pub fn linear(&self) -> Box<[u32]> {
        self.data.clone()
    }

    pub fn templated(&self) -> Box<[u32]> {
        if self.board_type == BoardType::Linear {
            return self.linear();
        }
        if self.board_type == BoardType::Snail {
            return self.snail();
        }
        unreachable!();
    }
}

impl Board {
    fn snail(&self) -> Box<[u32]> {
        let mut snail: Vec<u32> = Vec::new();
        let mut min = 0;
        let mut max = self.line_size;

        while min < max {
            for x in min..max {
                snail.push(self.data[min * self.line_size + x]);
            }
            for y in (min + 1)..(max - 1) {
                snail.push(self.data[y * self.line_size + max - 1])
            }
            for x in (min..max).rev() {
                snail.push(self.data[(max - 1) * self.line_size + x]);
            }
            for y in ((min + 1)..(max - 1)).rev() {
                snail.push(self.data[y * self.line_size + min])
            }
            min += 1;
            max -= 1;
        }
        snail.dedup();
        snail.into_boxed_slice()
    }
}
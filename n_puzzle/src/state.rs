use std::rc::Rc;
use board::Board;
use tile_move::Move;
use heuristic::Heuristic;
use std::cmp::Ordering;


#[derive(Clone, Eq, PartialEq, Debug)]
pub struct State {
    pub cost: usize,
    pub distance: usize,
    pub board: Board,
    pub parent: Option<Rc<State>>,
}

impl State {
    pub fn children<H: Heuristic>(&self, expected: &Board, heuristic: &H) -> Vec<State> {
        let parent = Rc::new(self.clone());
        self.board.children().into_iter().map(|board| Self {
            cost: self.cost + 1,
            distance: heuristic.distance(&board),
            board: board,
            parent: Some(parent.clone())
        }).collect()
    }

    pub fn build_path(&self) -> Vec<Move> {
        fn precedent_move(state: &State, path: &mut Vec<Move>) {
            if let Some(ref parent) = state.parent {
                precedent_move(parent, path);
                let move_ = Move::new(&parent.board, &state.board);
                path.push(move_);
            }
        }
        let mut path = Vec::new();
        precedent_move(self, &mut path);
        path
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        // Notice that the we flip the ordering on costs.
        Some(other.distance.cmp(&self.distance)
                .then(other.cost.cmp(&self.cost)))
    }
}
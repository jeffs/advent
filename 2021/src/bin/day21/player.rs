use crate::wrap;

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Player {
    pub position: usize,
    pub score: usize,
}

impl Player {
    pub fn advance(&mut self, distance: usize) {
        self.position = wrap::add(self.position, distance, 10);
        self.score += self.position;
    }
}

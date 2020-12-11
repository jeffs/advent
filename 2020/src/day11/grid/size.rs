pub struct Size {
    pub height: usize,
    pub width: usize,
}

impl Size {
    pub fn area(&self) -> usize {
        self.height * self.width
    }
}

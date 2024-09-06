use std::collections::VecDeque;

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

pub struct Snake {
    pub dir: Direction,
    body: VecDeque<(usize,usize)>
}

impl Snake {
    pub fn new() -> Snake {
        Snake{dir: Direction::Right, body: VecDeque::new()}
    }

    pub fn push_front(&mut self, x: usize, y: usize) {
        self.body.push_front((x,y));
    }

    pub fn pop_back(&mut self) -> Option<(usize, usize)> {
        self.body.pop_back()
    }

    pub fn head(&self) -> Option<(usize, usize)> {
        self.body.front().copied()
    }
}
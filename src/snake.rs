use std::collections::VecDeque;
use std::time::{Duration,Instant};

#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone,Copy)]
pub struct SnakeSegment {
    pub previous_position: (f64,f64),
    pub current_position: (f64,f64),
    pub removal_time: Option<Instant>,
}

pub struct Snake {
    pub dir: Direction,
    pub body: VecDeque<SnakeSegment>,
    pub removed_segments: Vec<SnakeSegment>,
}

impl Snake {
    pub fn new() -> Snake {
        Snake {
            dir: Direction::Right,
            body: VecDeque::new(),
            removed_segments: Vec::new(),
        }
    }

    pub fn push_front(&mut self, x: f64, y: f64) {
        let previous_position = if let Some(head) = self.body.front() {
            head.current_position
        } else {
            (x, y)
        };
        let segment = SnakeSegment {
            previous_position,
            current_position: (x,y),
            removal_time: None,
        };
        self.body.push_front(segment);
    }

    pub fn pop_back(&mut self) -> Option<SnakeSegment> {
        if let Some(tail_segment) = self.body.pop_back() {
            self.removed_segments.push(tail_segment.clone());
            Some(tail_segment)
        } else {
            None
        }
    }

    pub fn head(&self) -> Option<(f64, f64)> {
        self.body.front().map(|segment| segment.current_position)
    }

    pub fn cleanup_removed_segments(&mut self, update_duration: Duration) {
        let now = Instant::now();
        self.removed_segments.retain(|segment| {
            if let Some(removal_time) = segment.removal_time {
                now.duration_since(removal_time) < update_duration
            } else {
                false
            }
        });
    }
}
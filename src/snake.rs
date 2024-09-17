use std::collections::VecDeque;
use std::time::{Duration,Instant};

// Enumeration to represent the snake's current direction of movement
#[derive(PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

// Represents a single segment of the snake
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

// Struct representing the snake with its direction and segments
impl Snake {

    // Initializes a new Snake struct
    pub fn new() -> Snake {
        Snake {
            dir: Direction::Right, // Default direction
            body: VecDeque::new(),
            removed_segments: Vec::new(),
        }
    }

    // Adds a new segment at the front of the snake (usually the head's new position)
    pub fn push_front(&mut self, x: f64, y: f64) {

        // Determine the previous position of the head for smooth animation
        let previous_position = if let Some(head) = self.body.front() {
            head.current_position
        } else {
            (x, y) // If the snake has no body, use the given x and y
        };

        // Create a new segment for the front (head) with the new and previous positions
        let segment = SnakeSegment {
            previous_position,
            current_position: (x,y),
            removal_time: None,
        };

        // Push the new segment to the front of the deque
        self.body.push_front(segment);
    }

    // Removes the segment from the back of the snake (usually the tail)
    pub fn pop_back(&mut self) -> Option<SnakeSegment> {

        // If the snake has a tail segment, remove and store it in removed_segments
        if let Some(tail_segment) = self.body.pop_back() {

            // Clone the removed segment for animations and store it in removed_segments
            self.removed_segments.push(tail_segment.clone());
            Some(tail_segment)
        } else {
            None // If the snake has no body, return None
        }
    }

    // Returns the position of the head of the snake
    pub fn head(&self) -> Option<(f64, f64)> {

        // Map the current position of the head (first segment) if it exists
        self.body.front().map(|segment| segment.current_position)
    }

    // Cleans up old removed segments (to be called after a certain duration)
    pub fn cleanup_removed_segments(&mut self, update_duration: Duration) {
        let now = Instant::now(); // Get current time for comparison

        // Retain only those segments whose removal time is within the update duration
        self.removed_segments.retain(|segment| {
            if let Some(removal_time) = segment.removal_time {
                // Keep segments if their removal time hasn't exceeded the update duration
                now.duration_since(removal_time) < update_duration
            } else {
                false // Remove segments that don't have a valid removal_time
            }
        });
    }
}
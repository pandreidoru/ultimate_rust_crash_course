use crate::frame::{Drawable, Frame};
use crate::{NUM_COLS, NUM_ROWS};
use rusty_time::Timer;
use std::cmp::max;
use std::time::Duration;

pub struct Invader {
    pub x: usize,
    pub y: usize,
}

pub struct Invaders {
    pub army: Vec<Invader>,
    move_timer: Timer,
    // Positive move to right, negative move to left
    direction: i32,
}

impl Invaders {
    pub fn new() -> Self {
        let mut army = Vec::new();

        for x in 0..NUM_COLS {
            for y in 0..NUM_ROWS {
                // Not on the X edges
                if (x > 1) && (x < NUM_COLS - 2)
                    // Not on first row
                    && (y > 0)
                    // 7 rows
                    && (y < 9)
                    // Only on even cols
                    && (x % 2 == 0)
                    // Only on even rows
                    && (y % 2 == 0)
                {
                    army.push(Invader { x, y })
                }
            }
        }

        Self {
            army,
            move_timer: Timer::new(Duration::from_millis(2000)),
            direction: 1,
        }
    }

    pub fn update(&mut self, delta: Duration) -> bool {
        self.move_timer.tick(delta);
        if self.move_timer.finished() {
            self.move_timer.reset();
            let mut downwards = false;

            if self.direction == -1 {
                let min_x = self.army.iter().map(|invader| invader.x).min().unwrap_or(0);
                // Move right and downwards if reached left margin
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true;
                }
            } else {
                let max_x = self.army.iter().map(|invader| invader.x).max().unwrap_or(0);
                // Move left and downwards if reached right margin
                if max_x == NUM_COLS - 1 {
                    self.direction = -1;
                    downwards = true;
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration().as_millis() - 250, 250);
                self.move_timer = Timer::new(Duration::from_millis(new_duration as u64));
                for invader in self.army.iter_mut() {
                    invader.y += 1;
                }
            } else {
                for invader in self.army.iter_mut() {
                    invader.x = ((invader.x as i32) + self.direction) as usize;
                }
            }

            return true;
        }
        false
    }
}

impl Drawable for Invaders {
    fn draw(&self, frame: &mut Frame) {
        for invader in self.army.iter() {
            frame[invader.x][invader.y] = if (self.move_timer.remaining().as_secs_f32()
                / self.move_timer.duration().as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            }
        }
    }
}

use std::time::Duration;

use crate::{
    frame::{Drawable, Frame},
    invaders::Invaders,
    settings::Settings,
    shot::Shot,
};

pub struct Player {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Player {
    pub fn new(settings: &Settings) -> Self {
        Self {
            x: settings.columns() / 2,
            y: settings.rows() - 1,
            shots: Vec::new(),
        }
    }
    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }
    pub fn move_right(&mut self, columns: usize) {
        if self.x < columns - 1 {
            self.x += 1;
        }
    }
    pub fn shoot(&mut self) -> bool {
        if self.shots.len() < 2 {
            self.shots.push(Shot::new(self.x, self.y - 1));
            true
        } else {
            false
        }
    }
    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }
    pub fn detect_hits(&mut self, invaders: &mut Invaders) -> bool {
        let mut hit_something = false;
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if invaders.kill_invader_at(shot.x, shot.y) {
                    hit_something = true;
                    shot.explode();
                }
            }
        }
        hit_something
    }
}

impl Drawable for Player {
    fn draw(&self, frame: &mut Frame) {
        frame[self.x][self.y] = "🦞";
        for shot in self.shots.iter() {
            shot.draw(frame);
        }
    }
}

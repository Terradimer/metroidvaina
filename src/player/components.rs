use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Body {
    pub height: f32,
    pub width: f32,
    pub collider_ref: Entity,
}

#[derive(Component)]
pub struct FacingDirection(f32);

impl FacingDirection {
    pub fn new() -> Self {
        FacingDirection(1.)
    }

    pub fn set(&mut self, dir: f32) {
        self.0 = dir.signum();
    }

    pub fn get(&self) -> f32 {
        self.0.signum()
    }
}

#[derive(Component)]
pub struct Grounded {
    in_state: bool,
}

impl Grounded {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self { in_state: false }
    }
}

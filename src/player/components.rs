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

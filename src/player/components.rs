use std::time::Duration;

use bevy::prelude::*;

use crate::time::resources::ScaledTime;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct UpperCollider;

#[derive(Component)]
pub struct LowerCollider;

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
pub struct InputFreeze(Timer);

impl InputFreeze {
    pub fn set(&mut self, seconds: f32) {
        self.0.set_duration(Duration::from_secs_f32(seconds));
        self.0.reset();
    }

    pub fn check(&self) -> bool {
        self.0.finished()
    }

    pub fn new() -> Self {
        InputFreeze(Timer::from_seconds(0., TimerMode::Once))
    }
}

pub fn tick_input_freeze(mut q_freeze: Query<&mut InputFreeze>, time: Res<ScaledTime>) {
    for mut freeze_timer in q_freeze.iter_mut() {
        freeze_timer.0.tick(time.delta);
    }
}

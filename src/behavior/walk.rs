//! Still needs a more complete transition into the behavior system, I got really tired


use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::{
    input::{buffer::InputBuffer, directions::InputDirection, inputs::Inputs},
    player::components::Player,
};

use super::crouch::Crouch;

#[derive(Component)]
pub struct Walk {
    stage: Stage,
    slowing_factor: f32,
    max_speed: f32,
    acceleration_factor: f32,
}

pub enum Stage {
    Dormant,
    Active,
    Slowing,
}

impl Walk {
    pub fn new(slowing_factor: f32, max_speed: f32, acceleration_factor: f32) -> Self {
        Self {
            stage: Stage::Dormant,
            slowing_factor,
            max_speed,
            acceleration_factor,
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}

pub fn walking_behavior_player(
    mut q_player: Query<(&mut LinearVelocity, &Crouch, &InputBuffer, &Walk), With<Player>>,
    time: Res<Time>,
) {
    for (mut vel, crouching, buffer, state) in q_player.iter_mut() {
        let x_input = buffer.this_frame().x();

        if !(x_input.abs() > 0.2)
            || vel.x.signum() * x_input.signum() < 0.
            || buffer.blocked(Inputs::Directional)
        {
            vel.x -= vel.x * state.slowing_factor * time.delta_seconds();
        }

        if crouching.check() || buffer.blocked(InputDirection::Down) {
            return;
        }

        vel.x = (vel.x
            + x_input * state.max_speed * state.acceleration_factor * time.delta_seconds())
        .clamp(-state.max_speed, state.max_speed);
    }
}

pub struct WalkBehavior;

impl Plugin for WalkBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, walking_behavior_player);
    }
}

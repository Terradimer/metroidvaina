//! Still needs a more complete transition into the behavior system, I got really tired

use avian2d::dynamics::rigid_body::LinearVelocity;
use bevy::prelude::*;

use crate::{
    characters::demo_player::DemoPlayer,
    input::{buffer::InputBuffer, inputs::Inputs},
};

use super::{crouch::Crouch, kick::Kick};

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

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn is_dormant(&self) -> bool {
        matches!(self.stage, Stage::Dormant)
    }
    
    pub fn max_speed(&self) -> f32 {
        self.max_speed
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}

pub fn walking_behavior_player(
    mut q_player: Query<
        (&mut LinearVelocity, &InputBuffer, &mut Walk, Option<&Kick>),
        With<DemoPlayer>,
    >,
    time: Res<Time>,
) {
    for (mut vel, buffer, mut state, kick) in &mut q_player {
        let x_input = buffer.current_frame().x();

        if kick.is_some_and(Kick::is_active) {
            continue;
        }

        if x_input.abs() < 0.2 && vel.x.abs() < 1. {
            state.stage = Stage::Dormant;
            continue;
        } else if !(x_input.abs() > 0.2)
            || vel.x.signum() * x_input.signum() < 0.
            || buffer.blocked(Inputs::Directional)
        {
            state.stage = Stage::Slowing;
            vel.x -= vel.x * state.slowing_factor * time.delta_seconds();
        } else {
            state.stage = Stage::Active;
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

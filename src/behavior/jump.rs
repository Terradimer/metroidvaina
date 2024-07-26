use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::input::{buffer::InputBuffer, inputs::Inputs};
use crate::state::grounded::Grounded;

use super::crouch::Crouch;
use super::slide::Slide;

#[derive(Component)]
pub struct Jump {
    has_air_jumped: bool,
    force: f32,
    stage: Stage,
}

pub enum Stage {
    Dormant,
    Active,
}

impl Jump {
    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    pub fn new(jump_force: f32) -> Self {
        Self {
            has_air_jumped: false,
            force: jump_force,
            stage: Stage::Dormant,
        }
    }

    pub fn has_air_jumped(&self) -> bool {
        self.has_air_jumped
    }

    pub fn force(&self) -> f32 {
        self.force
    }

    pub fn reset_air_jump(&mut self) {
        self.has_air_jumped = false;
    }
}

pub fn jumping_behavior_player(
    mut q_state: Query<(
        Option<&Crouch>,
        Option<&Slide>,
        &Grounded,
        &mut LinearVelocity,
        &mut Jump,
        &mut InputBuffer,
    )>,
) {
    for (o_crouch, o_slide, grounded, mut vel, mut state, mut input_buffer) in q_state.iter_mut() {
        if o_crouch.is_some_and(Crouch::check) && o_slide.is_some() {
            return;
        }

        match state.stage {
            Stage::Dormant
                if (!state.has_air_jumped || grounded.check())
                    && input_buffer
                        .query()
                        .contains(Inputs::Jump.just_pressed())
                        .within_timeframe(Duration::from_millis(200))
                        .consume() =>
            {
                state.has_air_jumped = !grounded.check();

                state.set_stage(Stage::Active);
                vel.y = state.force;
                return;
            }
            Stage::Active if input_buffer.is(Inputs::Jump.released()) || vel.y < 0. => {
                state.set_stage(Stage::Dormant);
                vel.y /= 2.;
            }
            _ => {}
        }
    }
}

pub struct JumpBehavior;

impl Plugin for JumpBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, jumping_behavior_player);
    }
}

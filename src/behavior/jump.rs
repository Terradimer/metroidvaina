use bevy::prelude::*;
use avian2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::{resources::InputBlocker, Inputs},
    player::components::Grounded,
};

use super::crouch::Crouch;

#[derive(Component)]
pub struct Jumping {
    pub has_air_jumped: bool,
    pub jump_force: f32,
    stage: Stage,
}

pub enum Stage {
    Dormant,
    Active,
}

impl Jumping {
    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    pub fn new(jump_force: f32) -> Self {
        Self {
            has_air_jumped: false,
            jump_force,
            stage: Stage::Dormant,
        }
    }

    pub fn reset_air_jump(&mut self) {
        self.has_air_jumped = false
    }
}

pub fn jumping_behavior_player(
    mut q_state: Query<(&mut LinearVelocity, Option<&Crouch>, &Grounded, &mut Jumping)>,
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
) {
    for (mut vel, o_crouching, grounded, mut state) in q_state.iter_mut() {

        if let Some(crouching) = o_crouching {
            if crouching.check() && state.has_air_jumped {
                return;
            }
        }

        match state.stage {
            Stage::Dormant => {
                if grounded.check() {
                    state.reset_air_jump();
                }

                if input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                    && !state.has_air_jumped
                {
                    if let Some(input_axis) = input.clamped_axis_pair(&Inputs::Directional) {
                        if input_axis.y() < 0. && !state.has_air_jumped {
                            return;
                        }
                    };

                    if !grounded.check() {
                        state.has_air_jumped = true;
                    }

                    state.set_stage(Stage::Active);
                    vel.y = state.jump_force;
                    return;
                }
            }
            Stage::Active if !input.pressed(&Inputs::Jump) || vel.y < 0. => {
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

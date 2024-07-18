use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::{resources::InputBlocker, Inputs},
    player::components::Grounded,
};

use super::crouch::Crouch;

#[derive(Component)]
pub struct Jump {
    pub has_air_jumped: bool,
    pub force: f32,
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

    pub fn reset_air_jump(&mut self) {
        self.has_air_jumped = false
    }
}

pub fn jumping_behavior_player(
    mut q_state: Query<(Option<&Crouch>, &Grounded, &mut LinearVelocity, &mut Jump)>,
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
) {
    for (o_crouching, grounded, mut vel, mut state) in q_state.iter_mut() {
        if o_crouching.is_some_and(|crouching| crouching.check()) && state.has_air_jumped {
            return;
        }

        match state.stage {
            Stage::Dormant
                if input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                    && (!state.has_air_jumped || grounded.check()) =>
            {
                state.has_air_jumped = !grounded.check();

                state.set_stage(Stage::Active);
                vel.y = state.force;
                return;
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

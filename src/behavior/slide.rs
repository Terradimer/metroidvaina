use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::collision_groups::Group;
use crate::enemies::Enemy;
use crate::shape_intersections::ShapeIntersections;
use crate::{
    input::{resources::InputBlocker, Inputs},
    player::components::{Body, FacingDirection, Player},
};

use super::crouch::Crouch;

#[derive(Component)]
pub struct Slide {
    stage_timer: Timer,
    stage: Stage,
    has_hit: bool,
    speed: f32,
}

pub enum Stage {
    Dormant,
    Accelerate,
    Settle,
}

impl Slide {
    pub fn new(speed: f32) -> Self {
        Self {
            stage_timer: Timer::from_seconds(0., TimerMode::Once),
            stage: Stage::Dormant,
            has_hit: false,
            speed,
        }
    }

    pub fn stage(&self) -> &Stage {
        &self.stage
    }

    pub fn set_stage(&mut self, stage: Stage) {
        match stage {
            Stage::Accelerate { .. } => {
                self.has_hit = false;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.15));
                self.stage_timer.reset();
            }
            Stage::Settle => {
                self.stage_timer.set_duration(Duration::from_secs_f32(0.3));
                self.stage_timer.reset();
            }
            _ => {}
        }
        self.stage = stage;
    }
}

fn sliding_handler_player(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_player: Query<
        (
            &mut LinearVelocity,
            &FacingDirection,
            &Crouch,
            &Body,
            &mut Slide,
            &Transform,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut shape_intersections: ShapeIntersections,
    q_enemy: Query<&Enemy>,
) {
    for (mut velocity, direction, crouching, body, mut state, transform) in q_player.iter_mut() {
        let timer_finished = state.stage_timer.tick(time.delta()).finished();

        match state.stage {
            Stage::Dormant => {
                if crouching.check()
                    && input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                {
                    input_blocker.block_many(Inputs::all_actions());
                    state.set_stage(Stage::Accelerate);
                }
            }
            Stage::Accelerate  if timer_finished => {
                state.set_stage(Stage::Settle);
            }
            Stage::Accelerate => {
                velocity.x = state.speed * direction.get();

                if state.has_hit {
                    return;
                }

                if let Some(other) = shape_intersections.shape_intersections(&Collider::rectangle(body.height / 2., body.height / 4.), transform.translation.xy() + Vec2::new(body.height / 4. * direction.get(), -body.height / 3.), 0., Group::Hurtbox.into()).iter().filter(|x| q_enemy.get(**x).is_ok()).next()
                {
                    println!("Slide-kicked into: {other:?}");
                    state.has_hit = true;
                }
            }
            Stage::Settle if timer_finished => {
                input_blocker.clear();
                state.set_stage(Stage::Dormant);
            }
            _ => {}
        }
    }
}

pub struct SlidingBehavior;

impl Plugin for SlidingBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, sliding_handler_player);
    }
}

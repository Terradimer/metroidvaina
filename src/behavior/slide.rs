use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::collision_groups::{CollisionGroup, ENEMY};
use crate::state::facing_direction::FacingDirection;
use crate::{
    characters::demo_player::DemoPlayer,
    characters::Body,
    input::{buffer::InputBuffer, inputs::Inputs},
    shape_intersections::ShapeIntersections,
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

    pub fn check(&self) -> bool {
        !matches!(&self.stage, Stage::Dormant)
    }

    pub fn set_stage(&mut self, stage: Stage) {
        match stage {
            Stage::Accelerate { .. } => {
                self.has_hit = false;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.15));
                self.stage_timer.reset();
            }
            Stage::Settle => {
                self.stage_timer.set_duration(Duration::from_secs_f32(0.35));
                self.stage_timer.reset();
            }
            _ => {}
        }
        self.stage = stage;
    }
}

fn sliding_handler_player(
    mut q_player: Query<
        (
            &mut LinearVelocity,
            &mut InputBuffer,
            &FacingDirection,
            &Crouch,
            &Body,
            &mut Slide,
            &Transform,
        ),
        With<DemoPlayer>,
    >,
    time: Res<Time>,
    mut shape_intersections: ShapeIntersections,
) {
    for (mut velocity, mut buffer, direction, crouching, body, mut state, transform) in
        q_player.iter_mut()
    {
        let timer_finished = state.stage_timer.tick(time.delta()).finished();

        match state.stage {
            Stage::Dormant
                if crouching.check()
                    && buffer
                        .query()
                        .contains(Inputs::Jump.just_pressed())
                        .within_timeframe(Duration::from_millis(200))
                        .consume() =>
            {
                buffer.block_all();
                state.set_stage(Stage::Accelerate);
            }
            Stage::Accelerate if timer_finished => {
                state.set_stage(Stage::Settle);
            }
            Stage::Accelerate if state.has_hit => {
                velocity.x = state.speed * direction.get();
            }
            Stage::Accelerate => {
                velocity.x = state.speed * direction.get();

                if let Some(other) = shape_intersections
                    .shape_intersections(
                        &Collider::rectangle(body.height / 2., body.height / 4.),
                        transform.translation.xy()
                            + Vec2::new(body.height / 4. * direction.get(), -body.height / 3.),
                        0.,
                        CollisionGroup::filter(ENEMY),
                    )
                    .first()
                {
                    println!("Slide-kicked into: {other:?}");
                    state.has_hit = true;
                }
            }
            Stage::Settle if timer_finished => {
                buffer.clear_blocker();
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

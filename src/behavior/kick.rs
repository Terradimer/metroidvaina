use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;

use crate::input::buffer::InputBuffer;
use crate::input::directions::InputDirection;
use crate::input::inputs::Inputs;
use crate::shape_intersections::ShapeIntersections;

use crate::state::facing_direction::FacingDirection;
use crate::state::grounded::Grounded;
use crate::{
    collision_groups::*,
    player::components::{Body, Player},
};

use super::jump::{self, jumping_behavior_player, Jump};

#[derive(Component)]
pub struct Kick {
    stage: Stage,
    kick_speed: f32,
}

pub enum Stage {
    Dormant,
    Active,
}

impl Kick {
    pub fn new(kick_speed: f32) -> Self {
        Self {
            stage: Stage::Dormant,
            kick_speed,
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}

pub fn kicking_behavior_player(
    mut q_state: Query<
        (
            &mut LinearVelocity,
            &mut InputBuffer,
            &mut Jump,
            &mut Kick,
            &Body,
            &Grounded,
            &Transform,
            &mut GravityScale,
            &FacingDirection,
        ),
        With<Player>,
    >,
    mut shape_intersections: ShapeIntersections,
) {
    for (
        mut vel,
        mut buffer,
        mut jump,
        mut state,
        body,
        grounded,
        transform,
        mut gravity,
        facing_direction,
    ) in q_state.iter_mut()
    {
        match state.stage {
            Stage::Dormant if jump.has_air_jumped() => {
                let x = match buffer
                    .query()
                    .within_timeframe(Duration::from_millis(200))
                    .contains_any(vec![Inputs::Jump.just_pressed()])
                    .contains_any(
                        InputDirection::DownRight.roll_clockwise(InputDirection::DownLeft),
                    )
                    .consume_recent()
                {
                    Some(frame) => frame.x(),
                    None => return,
                };

                buffer.block_all();
                state.set_stage(Stage::Active);
                gravity.0 = 0.;

                if vel.x.signum() * x.signum() < -0.2 || vel.x.abs() < state.kick_speed {
                    vel.x = state.kick_speed * x.abs().ceil().copysign(x) * 0.9;
                }

                vel.y = -state.kick_speed;
                *vel = LinearVelocity(vel.normalize_or_zero() * vel.length());
            }
            Stage::Active if grounded.check() => {
                state.stage = Stage::Dormant;
                gravity.0 = 1.;
                buffer.clear_blocker();
            }
            Stage::Active => {
                if let Some(other) = shape_intersections
                    .shape_intersections(
                        &Collider::rectangle(body.width, body.height / 2.),
                        transform.translation.xy()
                            + Vec2::new(
                                body.width / 4. * facing_direction.get(),
                                -body.height / 4.,
                            ),
                        0.,
                        CollisionGroup::filter(ENEMY),
                    )
                    .first()
                {
                    println!("Kicked: {other:?}");
                    state.set_stage(Stage::Dormant);
                    buffer.clear_blocker();
                    gravity.0 = 1.;

                    jump.set_stage(jump::Stage::Active);
                    vel.y = jump.force();
                    jump.reset_air_jump();
                }
            }
            _ => {}
        }
    }
}

pub struct KickingBehavior;

impl Plugin for KickingBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            kicking_behavior_player.before(jumping_behavior_player),
        );
    }
}

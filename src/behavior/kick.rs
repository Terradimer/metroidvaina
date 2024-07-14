use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::enemies::Enemy;
use crate::shape_intersections::ShapeIntersections;
use crate::{
    collision_groups::*,
    input::{resources::InputBlocker, Inputs},
    player::components::{Body, Grounded, Player},
};

use super::jump::Jumping;

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
    pub fn new() -> Self {
        Self {
            stage: Stage::Dormant,
            kick_speed: 2200.,
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }
}

pub fn kicking_behavior_player(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_state: Query<
        (
            &mut LinearVelocity,
            &mut Jumping,
            &mut Kick,
            &Body,
            &Grounded,
            &Transform,
        ),
        With<Player>,
    >,
    mut shape_intersections: ShapeIntersections,
    q_enemy: Query<&Enemy>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    for (mut vel, mut jumping, mut state, body, grounded, transform) in q_state.iter_mut() {
        match state.stage {
            Stage::Dormant => {
                if input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                    && jumping.has_air_jumped
                    && move_axis.y < 0.
                {
                    input_blocker.block_many(Inputs::all_actions());
                    state.set_stage(Stage::Active);

                    if vel.x.signum() * move_axis.x.signum() < -0.2
                        || vel.x.abs() < state.kick_speed
                    {
                        vel.x =
                            state.kick_speed * move_axis.x.abs().ceil().copysign(move_axis.x) * 1.1;
                    }

                    (*vel).y = -state.kick_speed;
                    *vel = avian2d::prelude::LinearVelocity(vel.normalize() * vel.length());
                }
            }
            Stage::Active if grounded.check() => {
                state.stage = Stage::Dormant;
                input_blocker.clear()
            }
            Stage::Active => {
                if let Some(other) = shape_intersections.shape_intersections(&Collider::rectangle(body.width, body.height / 2.), transform.translation.xy() + Vec2::new(body.width / 4. * move_axis.x, -body.height / 4.), 0., Group::Hurtbox.into()).iter().filter(|x| q_enemy.get(**x).is_ok()).next()
                {
                    println!("Kicked: {other:?}");
                    state.set_stage(Stage::Dormant);
                    input_blocker.clear();

                    jumping.set_stage(super::jump::Stage::Active);
                    vel.y = jumping.jump_force;
                    jumping.reset_air_jump();
                    println!("Yump");
                }
            }
        }
    }
}

pub struct KickingBehavior;

impl Plugin for KickingBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, kicking_behavior_player);
    }
}

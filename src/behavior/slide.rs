use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    collision_groups::{CollisionGroups, Group},
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
    Accelerate { collider: Entity },
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

    pub fn spawn_collider(
        commands: &mut Commands,
        parent: Entity,
        direction: f32,
        height: f32,
    ) -> Entity {
        let collider = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(Vec3 {
                    x: height / 4. * direction,
                    y: -height / 3.,
                    z: 0.,
                })),
                Collider::rectangle(height / 2., height / 4.),
                Sensor,
                CollisionGroups::hitbox(&[Group::Enemy]),
            ))
            .id();

        commands.entity(parent).add_child(collider);
        collider
    }
}

fn sliding_handler_player(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_player: Query<
        (
            Entity,
            &mut LinearVelocity,
            &FacingDirection,
            &Crouch,
            &Body,
            &mut Slide,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    q_colliding_entities: Query<&CollidingEntities>,
) {
    for (entity, mut velocity, direction, crouching, body, mut state) in q_player.iter_mut() {
        let timer_finished = state.stage_timer.tick(time.delta()).finished();

        match state.stage {
            Stage::Dormant => {
                if crouching.check()
                    && input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                {
                    input_blocker.block_many(Inputs::all_actions());
                    state.set_stage(Stage::Accelerate {
                        collider: Slide::spawn_collider(
                            &mut commands,
                            entity,
                            direction.get(),
                            body.height,
                        ),
                    });
                }
            }
            Stage::Accelerate { collider } if timer_finished => {
                commands.entity(collider).despawn();
                state.set_stage(Stage::Settle);
            }
            Stage::Accelerate { collider } => {
                velocity.x = state.speed * direction.get();

                if state.has_hit {
                    return;
                }

                if let Some(other) = q_colliding_entities
                    .get(collider)
                    .ok()
                    .and_then(|x| x.0.iter().next())
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

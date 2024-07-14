use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    collision_groups::*,
    input::{resources::InputBlocker, Inputs},
    player::components::{FacingDirection, Grounded, Player},
};

#[derive(Component)]
pub struct DemoSlash {
    stage: Stage,
    stage_timer: Timer,
    has_hit: bool,
}

pub enum Stage {
    Dormant,
    Windup,
    Active { colliders: Vec<Entity> },
    Settle,
}

impl DemoSlash {
    pub fn new() -> Self {
        Self {
            has_hit: false,
            stage: Stage::Dormant,
            stage_timer: Timer::from_seconds(0., TimerMode::Once),
        }
    }

    pub fn set_stage(&mut self, next: Stage) {
        match next {
            Stage::Dormant => {
                self.stage = next;
            }
            Stage::Windup => {
                self.stage = next;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.1));
            }
            Stage::Active { .. } => {
                self.has_hit = false;
                self.stage = next;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.3));
            }
            Stage::Settle => {
                self.stage = next;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.1));
            }
        }
        self.stage_timer.reset();
    }

    pub fn spawn_colliders(
        commands: &mut Commands,
        slash_size: f32,
        parent: Entity,
        direction: f32,
    ) -> Vec<Entity> {
        let mut colliders = Vec::with_capacity(3);

        for i in 0..3 {
            let is_side_collider = i % 2 == 0;

            let collider_size = if is_side_collider {
                slash_size / 2.25
            } else {
                slash_size
            };

            colliders.push(
                commands
                    .spawn((
                        SpatialBundle::from_transform(Transform::from_translation(
                            if is_side_collider {
                                Vec3 {
                                    y: collider_size
                                        + slash_size
                                        + i as f32 * (-collider_size - slash_size),
                                    x: slash_size * direction * 2.,
                                    z: 0.,
                                }
                            } else {
                                Vec3 {
                                    x: collider_size * 2.5 * direction,
                                    ..Vec3::ZERO
                                }
                            },
                        )),
                        Collider::rectangle(collider_size, collider_size),
                        Sensor,
                        CollisionGroups::hitbox(&[Group::Enemy]),
                        Name::new("DemoSlashSensor"),
                    ))
                    .id(),
            );
        }
        commands.entity(parent).push_children(&colliders);
        colliders
    }
}

pub fn demo_slash_player_behavior(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_state: Query<
        (
            Entity,
            &mut LinearVelocity,
            &mut DemoSlash,
            &Grounded,
            &FacingDirection,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut commands: Commands,
    q_colliding_entities: Query<&CollidingEntities>,
) {
    for (entity, mut vel, mut state, grounded, direction) in q_state.iter_mut() {
        let timer_finished = state.stage_timer.tick(time.delta()).finished();

        match &state.stage {
            Stage::Dormant
                if input.just_pressed(&Inputs::Primary)
                    && !input_blocker.check(Inputs::Primary) =>
            {
                state.set_stage(Stage::Windup);
                input_blocker.block_many(Inputs::all_actions());

                if grounded.check() {
                    vel.x = 0.;
                }
            }
            Stage::Windup if timer_finished => {
                state.set_stage(Stage::Active {
                    colliders: DemoSlash::spawn_colliders(
                        &mut commands,
                        25.,
                        entity,
                        direction.get(),
                    ),
                });
            }
            Stage::Active { colliders } if timer_finished => {
                let colliders_to_despawn = colliders.clone();

                // Despawn the colliders
                for &collider in &colliders_to_despawn {
                    commands.entity(collider).despawn_recursive();
                }

                state.set_stage(Stage::Settle);
                input_blocker.clear();
            }
            Stage::Active { colliders } if !state.has_hit => {
                if let Some(other) = colliders
                    .iter()
                    .flat_map(|x| q_colliding_entities.get(*x))
                    .flat_map(|x| x.0.iter().next())
                    .next()
                {
                    println!("Slashed: {other:?}",);
                    state.has_hit = true;
                };
            }
            Stage::Settle if timer_finished => {
                state.set_stage(Stage::Dormant);
            }
            _ => {}
        }
    }
}

pub struct SlashingBehavior;

impl Plugin for SlashingBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_slash_player_behavior);
    }
}

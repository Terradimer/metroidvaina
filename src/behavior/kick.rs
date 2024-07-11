use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    collision_groups::Groups,
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
    Active { collider: Entity },
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

    pub fn spawn_collider(
        commands: &mut Commands,
        parent: Entity,
        height: f32,
        width: f32,
        direction: f32,
    ) -> Entity {
        let collider = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_xyz(
                    width / 4. * direction,
                    -height / 4.,
                    0.,
                )),
                Groups::hitbox(Groups::ENEMY),
                Collider::cuboid(width / 2., height / 4.),
                Sensor,
            ))
            .id();
        commands.entity(parent).add_child(collider);
        collider
    }

    pub fn get_collision(&self, rapier_context: &RapierContext) -> Option<Entity> {
        let collider = match self.stage {
            Stage::Dormant => return None,
            Stage::Active { collider } => collider,
        };

        for (entity1, entity2, _) in rapier_context
            .intersection_pairs_with(collider)
            .filter(|(_, _, intersecting)| *intersecting)
        {
            if entity1 != collider {
                return Some(entity1);
            } else {
                return Some(entity2);
            }
        }
        None
    }
}

pub fn kicking_behavior_player(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_state: Query<
        (
            Entity,
            &mut Velocity,
            &mut Jumping,
            &mut Kick,
            &Body,
            &Grounded,
        ),
        With<Player>,
    >,
    rapier_context: Res<RapierContext>,
    mut commands: Commands,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    for (entity, mut velocity, mut jumping, mut state, body, grounded) in q_state.iter_mut() {
        let vel = &mut velocity.linvel;
        match state.stage {
            Stage::Dormant => {
                if input.just_pressed(&Inputs::Jump)
                    && !input_blocker.check(Inputs::Jump)
                    && jumping.has_air_jumped
                    && move_axis.y < 0.
                {
                    input_blocker.block_many(Inputs::all_actions());
                    state.set_stage(Stage::Active {
                        collider: Kick::spawn_collider(
                            &mut commands,
                            entity,
                            body.height,
                            body.width,
                            move_axis.x,
                        ),
                    });

                    if vel.x.signum() * move_axis.x.signum() < -0.2
                        || vel.x.abs() < state.kick_speed
                    {
                        vel.x =
                            state.kick_speed * move_axis.x.abs().ceil().copysign(move_axis.x) * 1.1;
                    }

                    vel.y = -state.kick_speed;
                    *vel = vel.normalize() * vel.length();
                }
            }
            Stage::Active { collider } => {
                if let Some(other) = state.get_collision(&rapier_context) {
                    println!("Kicked: {other:?}");
                    state.stage = Stage::Dormant;
                    input_blocker.clear();

                    commands.entity(collider).despawn();
                    jumping.set_stage(super::jump::Stage::Active);
                    vel.y = jumping.jump_force;
                    jumping.reset_air_jump();
                }

                if grounded.check() {
                    state.stage = Stage::Dormant;

                    commands.entity(collider).despawn();
                    input_blocker.clear()
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

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

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
                CollisionGroups::hitbox(&[Group::Enemy]),
                Collider::rectangle(width, height / 2.),
                Sensor,
                Name::new("KickSensor"),
            ))
            .id();
        commands.entity(parent).add_child(collider);
        collider
    }
}

pub fn kicking_behavior_player(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_state: Query<
        (
            Entity,
            &mut LinearVelocity,
            &mut Jumping,
            &mut Kick,
            &Body,
            &Grounded,
        ),
        With<Player>,
    >,
    mut commands: Commands,
    q_colliding_entities: Query<&CollidingEntities>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    for (entity, mut vel, mut jumping, mut state, body, grounded) in q_state.iter_mut() {
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

                    (*vel).y = -state.kick_speed;
                    *vel = avian2d::prelude::LinearVelocity(vel.normalize() * vel.length());
                }
            }
            Stage::Active { collider } if grounded.check() => {
                state.stage = Stage::Dormant;

                commands.entity(collider).despawn();
                input_blocker.clear()
            }
            Stage::Active { collider } => {
                if let Some(other) = q_colliding_entities
                    .get(collider)
                    .ok()
                    .and_then(|x| x.0.iter().next())
                {
                    println!("Kicked: {other:?}");
                    state.set_stage(Stage::Dormant);
                    input_blocker.clear();

                    commands.entity(collider).despawn_recursive();
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

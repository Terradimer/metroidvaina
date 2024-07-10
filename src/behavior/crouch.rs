use crate::collision_groups::Groups;
use bevy::prelude::*;
use bevy_rapier2d::{
    geometry::{Collider, CollisionGroups, Sensor},
    plugin::RapierContext,
};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::{resources::InputBlocker, Inputs},
    player::{
        components::{Body, Player},
        state::Grounded,
    },
};

use super::slide::Slide;

#[derive(Component)]
pub struct Crouch {
    stage: Stage,
}

pub enum Stage {
    Standing,
    Crouching {
        stuck_check_collider: Entity,
        collider_storage: Entity,
    },
}

impl Crouch {
    pub fn new() -> Self {
        Self {
            stage: Stage::Standing,
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
    }

    pub fn spawn_collision_collider(
        commands: &mut Commands,
        parent: Entity,
        height: f32,
        width: f32,
    ) -> Entity {
        let collider_ref = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0., -height / 4., 0.)),
                Groups::collision(),
                Collider::cuboid(width / 2., height / 4.),
            ))
            .id();

        commands.entity(parent).add_child(collider_ref);
        collider_ref
    }

    pub fn spawn_stuck_check(
        commands: &mut Commands,
        parent: Entity,
        height: f32,
        width: f32,
    ) -> Entity {
        let collider = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_translation(Vec3 {
                    y: height / 4.,
                    ..Vec3::ZERO
                })),
                Sensor,
                Groups::collision(),
                Collider::cuboid(width / 4., height / 4.),
            ))
            .id();

        commands.entity(parent).add_child(collider);
        collider
    }

    pub fn get_collision(&self, rapier_context: &RapierContext) -> bool {
        let collider_ref = match &self.stage {
            Stage::Crouching {
                stuck_check_collider,
                ..
            } => stuck_check_collider,
            _ => return false,
        };

        for (_, _, _) in rapier_context
            .intersection_pairs_with(*collider_ref)
            .filter(|(_, _, intersecting)| *intersecting)
        {
            return true;
        }

        false
    }

    pub fn check(&self) -> bool {
        match self.stage {
            Stage::Standing => false,
            Stage::Crouching { .. } => true,
        }
    }
}

pub fn crouching_behavior_player(
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
    mut q_player: Query<(Entity, &Grounded, &mut Body, &mut Crouch, Option<&Slide>), With<Player>>,
    mut q_collider: Query<&mut CollisionGroups>,
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
) {
    let axis_data = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let input_crouching =
        !input_blocker.check(Inputs::Directional) && axis_data.x.abs() < 0.2 && axis_data.y < 0.;

    for (entity, grounded, mut body, mut state, o_slide) in q_player.iter_mut() {
        let trying_crouch = if let Some(slide) = o_slide {
            (input_crouching && grounded.check())
                || !matches!(slide.stage(), super::slide::Stage::Dormant)
        } else {
            input_crouching && grounded.check()
        };

        match &state.stage {
            Stage::Standing if trying_crouch => {
                let Ok(mut collision_group) = q_collider.get_mut(body.collider_ref) else {
                    return;
                };

                // Make the crouch collision collider
                *collision_group = Groups::inactive();
                let new_collider = Crouch::spawn_collision_collider(
                    &mut commands,
                    entity,
                    body.height,
                    body.width,
                );

                state.set_stage(Stage::Crouching {
                    collider_storage: body.collider_ref,
                    stuck_check_collider: Crouch::spawn_stuck_check(
                        &mut commands,
                        entity,
                        body.height,
                        body.width,
                    ),
                });

                body.collider_ref = new_collider;
            }
            Stage::Crouching {
                stuck_check_collider,
                collider_storage,
            } if !trying_crouch => {
                if !state.get_collision(&rapier_context) {
                    commands.entity(body.collider_ref).despawn();
                    commands.entity(*stuck_check_collider).despawn();

                    body.collider_ref = *collider_storage;

                    let Ok(mut collision_group) = q_collider.get_mut(body.collider_ref) else {
                        return;
                    };
                    *collision_group = Groups::collision();

                    state.set_stage(Stage::Standing);
                }
            }
            _ => {}
        }
    }
}

pub struct CrouchBehavior;

impl Plugin for CrouchBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, crouching_behavior_player);
    }
}

// pub fn crouching_state_change(
//     mut q_state: Query<&mut Crouch>,
//     mut q_collider: Query<&mut CollisionGroups, With<UpperCollider>>,
//     q_stuck: Query<Entity, With<StuckCheck>>,
//     rapier_context: Res<RapierContext>,
// ) {
//     for (mut state, mut group, stuck_check) in izip!(
//         &mut q_state.iter_mut(),
//         &mut q_collider.iter_mut(),
//         q_stuck.iter()
//     ) {
//         if state.is_changed() || state.stuck {
//             if state.check() {
//                 group.memberships = Group::GROUP_3;
//             } else {
//                 if rapier_context
//                     .intersection_pairs_with(stuck_check)
//                     .any(|(_, _, intersecting)| intersecting)
//                 {
//                     state.in_state = true;
//                     state.stuck = true;
//                     group.memberships = Group::GROUP_3;
//                 } else {
//                     group.memberships = Group::GROUP_2;
//                     state.stuck = false;
//                 }
//             }
//         }
//     }
// }

// impl Crouching {
//     pub fn start(&mut self) {
//         if !self.in_state {
//             self.in_state = true
//         }
//     }

//     pub fn stop(&mut self) {
//         if self.in_state {
//             self.in_state = false
//         }
//     }

//     pub fn check(&self) -> bool {
//         self.in_state
//     }

//     pub fn new() -> Self {
//         Self {
//             in_state: false,
//             stuck: false,
//         }
//     }
// }
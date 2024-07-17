use crate::collision_groups::ENVIRONMENT;
use crate::shape_intersections::ShapeIntersections;
use crate::{collision_groups::CollisionGroup, player::components::Grounded};
use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    input::{resources::InputBlocker, Inputs},
    player::components::{Body, Player},
};

use super::slide::Slide;

#[derive(Component)]
pub struct Crouch {
    stage: Stage,
}

pub enum Stage {
    Standing,
    Crouching { collider_storage: Entity },
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
                CollisionGroup::collider(),
                Collider::rectangle(width, height / 2.),
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                Name::new("CrouchCollider"),
            ))
            .id();

        commands.entity(parent).add_child(collider_ref);
        collider_ref
    }

    pub fn check(&self) -> bool {
        match self.stage {
            Stage::Standing => false,
            Stage::Crouching { .. } => true,
        }
    }
}

pub fn crouching_behavior_player(
    mut q_player: Query<(Entity, Option<&Slide>, &Grounded, &mut Body, &mut Crouch), With<Player>>,
    mut collision_params: ParamSet<(Query<&mut CollisionLayers>, ShapeIntersections)>,
    q_transform: Query<&Transform>,
    input_blocker: Res<InputBlocker>,
    input: Res<ActionState<Inputs>>,
    mut commands: Commands,
) {
    let axis_data = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let input_crouching =
        !input_blocker.check(Inputs::Directional) && axis_data.x.abs() < 0.2 && axis_data.y < 0.;

    for (entity, slide, grounded, mut body, mut state) in q_player.iter_mut() {
        let trying_crouch =
            slide.is_some_and(|data| data.check()) || (input_crouching && grounded.check());

        match &state.stage {
            Stage::Standing if trying_crouch => {
                let mut q_collision_group = collision_params.p0();

                let Ok(mut body_collision_group) = q_collision_group.get_mut(body.collider_ref)
                else {
                    return;
                };

                *body_collision_group = CollisionGroup::inactive();

                state.set_stage(Stage::Crouching {
                    collider_storage: body.collider_ref,
                });

                body.collider_ref = Crouch::spawn_collision_collider(
                    &mut commands,
                    entity,
                    body.height,
                    body.width,
                );
            }
            Stage::Crouching { collider_storage } if !trying_crouch => {
                let Ok(transform) = q_transform.get(entity) else {
                    return;
                };

                if !collision_params
                    .p1()
                    .shape_intersections(
                        &Collider::rectangle(body.width / 2., body.height / 2.),
                        transform.translation.xy() + Vec2::new(0., body.height / 4.),
                        0.,
                        CollisionGroup::filter(ENVIRONMENT),
                    )
                    .is_empty()
                {
                    return;
                }

                commands.entity(body.collider_ref).despawn_recursive();

                body.collider_ref = *collider_storage;

                let mut q_collision_layers = collision_params.p0();
                let Ok(mut collision_group) = q_collision_layers.get_mut(body.collider_ref) else {
                    return;
                };
                *collision_group = CollisionGroup::collider();

                state.set_stage(Stage::Standing);
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

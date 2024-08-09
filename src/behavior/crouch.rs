use crate::{
    characters::demo_player::DemoPlayer,
    collision_groups::{CollisionGroup, ENVIRONMENT},
    input::{buffer::InputBuffer, directions::InputDirection},
    shape_intersections::ShapeIntersections,
    state::grounded::Grounded,
};
use avian2d::prelude::*;
use bevy::prelude::*;

use crate::characters::Body;

use super::slide::Slide;

#[derive(Component)]
pub struct Crouch {
    stage: Stage,
}

#[derive(Component)]
pub struct CrouchCollider;

pub enum Stage {
    Standing,
    Crouching { collider_reference: Entity },
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
                CollisionGroup::COLLIDER,
                Collider::rectangle(width, height / 2.),
                Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
                CrouchCollider,
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
    mut q_player: Query<
        (
            Entity,
            Option<&Slide>,
            &Grounded,
            &Body,
            &Transform,
            &mut LinearVelocity,
            &mut Crouch,
            &InputBuffer,
        ),
        With<DemoPlayer>,
    >,
    mut collision_params: ParamSet<(Query<&mut CollisionLayers>, ShapeIntersections)>,
    mut commands: Commands,
) {
    for (entity, slide, grounded, body, transform, mut vel, mut state, input) in &mut q_player {
        if slide.is_some_and(Slide::check) {
            continue;
        }

        match &state.stage {
            Stage::Standing if input.is(InputDirection::Down) && grounded.check() => {
                let mut q_collision_layers = collision_params.p0();
                let mut collider_group = match q_collision_layers.get_mut(body.collider_ref) {
                    Ok(layers) => layers,
                    Err(_) => continue,
                };

                *collider_group = CollisionGroup::INACTIVE;
                state.set_stage(Stage::Crouching {
                    collider_reference: Crouch::spawn_collision_collider(
                        &mut commands,
                        entity,
                        body.height,
                        body.width,
                    ),
                });
            }
            Stage::Crouching { collider_reference } if !input.is(InputDirection::Down) => {
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
                    continue;
                }

                commands.entity(*collider_reference).despawn_recursive();
                commands
                    .entity(entity)
                    .remove_children(&[*collider_reference]);

                let mut q_collision_layers = collision_params.p0();
                let mut collider_group = match q_collision_layers.get_mut(body.collider_ref) {
                    Ok(layers) => layers,
                    Err(_) => continue,
                };

                *collider_group = CollisionGroup::COLLIDER;
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

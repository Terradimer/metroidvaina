use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::collision_groups::CollisionGroup;

pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    color: Color,
    location: Vec2,
    size: Vec2,
    name: Name,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
            material: materials.add(color),
            transform: Transform::from_translation(location.extend(0.)),
            ..default()
        },
        CollisionGroup::ENVIRONMENT,
        RigidBody::Static,
        Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        Collider::rectangle(size.x, size.y),
        name,
    ));
}

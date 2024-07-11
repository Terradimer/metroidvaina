use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use avian2d::prelude::*;

use crate::collision_groups::Groups;

pub fn spawn_cube(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    color: Color,
    location: Vec2,
    size: Vec2,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: Mesh2dHandle(meshes.add(Rectangle::new(size.x, size.y))),
            material: materials.add(color),
            transform: Transform::from_translation(location.extend(0.)),
            ..default()
        },
        Groups::environment(),
        RigidBody::Static,
        Collider::rectangle(size.x, size.y),
    ));
}

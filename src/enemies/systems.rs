use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

use crate::collision_groups::Groups;

use super::components::Enemy;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.0))),
                material: materials.add(Color::RED),
                transform: Transform::from_xyz(400., -305., 0.),
                ..default()
            },
            Enemy,
            Groups::collision(),
            Collider::cuboid(25., 50.),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle::default(),
                Groups::hurtbox(Groups::ENEMY),
                Collider::cuboid(25., 50.),
            ));
        });
}

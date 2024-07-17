use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::collision_groups::{CollisionGroup, ENEMY};

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
                material: materials.add(Color::srgb(1., 0., 0.)),
                transform: Transform::from_xyz(400., -305., 0.),
                ..default()
            },
            Enemy,
            CollisionGroup::COLLIDER,
            Collider::rectangle(50., 100.),
            Name::new("TestDummyCollider"),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle::default(),
                Enemy,
                CollisionGroup::hurtbox(ENEMY),
                Collider::rectangle(50., 100.),
                Name::new("TestDummyHurtbox"),
            ));
        });
}

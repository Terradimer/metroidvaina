use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::prelude::*;

pub fn spawn_cube(
    mut commands: Commands<'_, '_>,
    mut meshes: ResMut<'_, Assets<Mesh>>,
    mut materials: ResMut<'_, Assets<ColorMaterial>>,
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
        Collider::cuboid(size.x / 2., size.y / 2.),
    ));
}

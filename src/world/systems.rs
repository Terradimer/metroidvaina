use bevy::prelude::*;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::functions::spawn_cube;

pub fn startup(
    mut commands: Commands,
    meshes: ResMut<Assets<Mesh>>,
    materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Ground
    spawn_cube(
        commands,
        meshes,
        materials,
        Color::GRAY,
        Vec2 {
            x: 0.,
            y: -WINDOW_HEIGHT / 2. + 25.,
        },
        Vec2 {
            x: WINDOW_WIDTH,
            y: 50.,
        },
    );
}

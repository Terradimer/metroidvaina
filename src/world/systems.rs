use bevy::prelude::*;
use bevy::color::palettes::css;

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::functions::spawn_cube;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn(Camera2dBundle::default());

    // Ground
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        css::GRAY.into(),
        Vec2 {
            x: 0.,
            y: -WINDOW_HEIGHT / 2. + 25.,
        },
        Vec2 {
            x: WINDOW_WIDTH,
            y: 50.,
        },
    );
    // outer walls
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        css::GRAY.into(),
        Vec2 {
            x: WINDOW_WIDTH / 2. - 25.,
            y: 0.,
        },
        Vec2 {
            x: 50.,
            y: WINDOW_HEIGHT,
        },
    );
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        css::GRAY.into(),
        Vec2 {
            x: -WINDOW_WIDTH / 2. + 25.,
            y: 0.,
        },
        Vec2 {
            x: 50.,
            y: WINDOW_HEIGHT,
        },
    );
    // middle
    spawn_cube(
        &mut commands,
        &mut meshes,
        &mut materials,
        css::GRAY.into(),
        Vec2 { x: 100., y: 110. },
        Vec2 {
            x: 50.,
            y: WINDOW_HEIGHT,
        },
    );
}

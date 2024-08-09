mod behavior;
mod camera;
mod characters;
mod collision_groups;
mod enemies;
mod input;
mod macros;
mod shape_intersections;
mod state;
mod world;

use avian2d::{
    debug_render::PhysicsDebugPlugin,
    prelude::Gravity,
    schedule::{Physics, TimestepMode},
    PhysicsPlugins,
};
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{PresentMode::AutoNoVsync, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use characters::CharactersPlugin;
use std::time::Duration;

use behavior::BehaviorPlugin;
use enemies::EnemiesPlugin;
use input::InputHandlerPlugin;
use state::StateHandlerPlugin;
use world::WorldPlugin;

pub const WINDOW_WIDTH: f32 = 1920. * 0.75;
pub const WINDOW_HEIGHT: f32 = 1080. * 0.75;

fn main() {
    App::new()
        .add_plugins((DefaultPlugins
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Metroidvainia Demo".to_string(),
                    resolution: WindowResolution::new(WINDOW_WIDTH, WINDOW_HEIGHT),
                    resizable: false,
                    present_mode: AutoNoVsync,
                    ..default()
                }),
                ..default()
            })
            .set(RenderPlugin {
                render_creation: RenderCreation::Automatic(WgpuSettings {
                    backends: Some(Backends::VULKAN),
                    ..default()
                }),
                ..default()
            })
            .set(ImagePlugin::default_nearest()),))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_plugins((
            PhysicsPlugins::default().with_length_unit(100.),
            PhysicsDebugPlugin::default(),
            // Project plugins1
            CharactersPlugin,
            InputHandlerPlugin,
            StateHandlerPlugin,
            WorldPlugin,
            EnemiesPlugin,
            BehaviorPlugin,
        ))
        .insert_resource(Gravity(Vec2::NEG_Y * 1000.0))
        .insert_resource(Time::new_with(Physics::from_timestep(
            TimestepMode::Fixed {
                delta: Duration::from_secs_f32(0.0001),
                overstep: Duration::from_secs_f32(0.001),
                max_delta_overstep: Duration::from_secs_f32(0.01),
            },
        )))
        .run();
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

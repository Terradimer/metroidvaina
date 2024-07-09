mod behavior;
mod camera;
mod collision_groups;
mod enemies;
mod input;
mod macros;
mod player;
mod time;
mod world;

use behavior::BehaviorPlugin;
use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{PresentMode::AutoNoVsync, WindowResolution},
};
use bevy_asset_loader::prelude::*;
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use enemies::EnemiesPlugin;
use input::InputHandlerPlugin;
use player::PlayerPlugin;
use time::TimeScalarPlugin;
use world::WorldPlugin;

pub const WINDOW_WIDTH: f32 = 1920. * 0.75;
pub const WINDOW_HEIGHT: f32 = 1080. * 0.75;

fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
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
                .set(ImagePlugin::default_nearest()),
            // Physics Plugins
            RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
        ))
        .init_state::<GameState>()
        .add_loading_state(
            LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
        )
        .add_plugins((
            // Project plugins
            InputHandlerPlugin,
            TimeScalarPlugin,
            WorldPlugin,
            PlayerPlugin,
            EnemiesPlugin,
            BehaviorPlugin,
        ))
        .run();
}

#[derive(PartialEq, Eq, Clone, Debug, Hash, Default, States)]
enum GameState {
    #[default]
    Loading,
    Playing,
}

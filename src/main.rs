use bevy::{
    prelude::*,
    render::{
        settings::{Backends, RenderCreation, WgpuSettings},
        RenderPlugin,
    },
    window::{PresentMode::AutoNoVsync, WindowResolution},
};
use bevy_rapier2d::{
    plugin::{NoUserData, RapierPhysicsPlugin},
    render::RapierDebugRenderPlugin,
};
use input::InputHandlerPlugin;
use player::PlayerPlugin;
use time::TimeScalarPlugin;
use world::WorldPlugin;

mod camera;
mod input;
mod macros;
mod player;
mod time;
mod world;

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
                }),
            // Physics Plugins
            // RapierDebugRenderPlugin::default(),
            RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.),
            // Project plugins
            InputHandlerPlugin,
            TimeScalarPlugin,
            WorldPlugin,
            PlayerPlugin,
        ))
        .run();
}

use bevy::prelude::*;

use self::systems::startup;

mod components;
mod resources;
mod systems;

pub use components::Enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

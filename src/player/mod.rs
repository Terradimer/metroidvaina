use bevy::prelude::*;

use self::systems::*;

pub mod components;
mod resources;
pub mod systems;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

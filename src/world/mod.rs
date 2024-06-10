use bevy::prelude::*;

mod functions;
mod systems;

use systems::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup);
    }
}

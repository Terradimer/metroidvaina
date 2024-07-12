use bevy::prelude::*;

use crate::GameState;

use self::systems::*;

pub mod components;
mod resources;
mod systems;

pub struct PlayerPlugin;

pub const PLAYER_MAX_SPEED: f32 = 500.;
pub const PLAYER_SLOWING_FACTOR: f32 = 4.3;
pub const PLAYER_ACCELERATION_FACTOR: f32 = 3.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                update_contact,
                update_facing_direction,
                horizontal_movement.after(update_contact),
            )
                .chain()
                .run_if(in_state(GameState::Playing)),
        );
    }
}

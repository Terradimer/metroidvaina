use bevy::prelude::*;

use self::{
    components::tick_input_freeze,
    state::{crouching_state_change, jumping_state_change, kicking_state_change},
    systems::*,
};

mod components;
mod resources;
mod state;
mod systems;

pub struct PlayerPlugin;

pub const PLAYER_MAX_SPEED: f32 = 500.;
pub const PLAYER_KICK_SPEED: f32 = 2200.;
pub const PLAYER_SLOWING_FACTOR: f32 = 4.3;
pub const PLAYER_ACCELERATION_FACTOR: f32 = 3.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                update_contact,
                tick_input_freeze,
                (horizontal_movement, jump, crouching, kicking),
                (
                    kicking_state_change,
                    jumping_state_change,
                    crouching_state_change,
                ),
            )
                .chain(),
        );
    }
}

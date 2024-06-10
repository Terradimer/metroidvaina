use bevy::prelude::*;

use self::{
    state::{jumping_state_change, kicking_state_change},
    systems::*,
};

mod components;
mod resources;
mod state;
mod systems;

pub struct PlayerPlugin;

pub const PLAYER_MAX_SPEED: f32 = 500.;
pub const PLAYER_KICK_SPEED: f32 = 2200.;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, startup).add_systems(
            Update,
            (
                update_contact,
                (horizontal_movement, jump, kicking),
                (kicking_state_change, jumping_state_change),
            )
                .chain(),
        );
    }
}

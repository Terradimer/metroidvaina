use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::{ConfigureLoadingState, LoadingStateConfig},
    LoadingStateAppExt,
};

use crate::GameState;

use self::resources::PlayerWalk;

pub mod resources;
mod systems;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.configure_loading_state(
            LoadingStateConfig::new(GameState::Loading).load_collection::<PlayerWalk>(),
        );
    }
}

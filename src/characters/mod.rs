use crate::GameState;
use bevy::prelude::*;
use demo_player::DemoPlayerPlugin;

pub mod demo_player;
pub mod resources;
mod systems;

#[derive(Component)]
pub struct Body {
    pub height: f32,
    pub width: f32,
    pub collider_ref: Entity,
}

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(DemoPlayerPlugin);
    }
}

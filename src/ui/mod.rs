use bevy::prelude::*;

use self::systems::*;
use self::resources::*;

mod systems;
mod resources;
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugins((
            bevy_egui::EguiPlugin,
            bevy_framepace::FramepacePlugin,
        ))
        .init_resource::<UiState>()
        .add_systems(Update, ui_system);
    }
}
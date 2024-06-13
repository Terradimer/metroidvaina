use bevy::app::{App, Plugin, Update};

pub mod resources;
mod systems;

pub struct TimeScalarPlugin;

impl Plugin for TimeScalarPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(resources::ScaledTime::default())
            .add_systems(Update, systems::update_scaled_time);
    }
}

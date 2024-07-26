use bevy::prelude::*;
use leafwing_input_manager::prelude::*;

use self::{buffer::update_buffers, inputs::Inputs};

// use self::buffers::update_inputs;

pub mod buffer;
pub mod directions;
mod input_frame;
pub mod input_query;
pub mod inputs;
pub mod blocker;

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Inputs>::default())
            .add_systems(Update, update_buffers)
            .init_resource::<ActionState<Inputs>>()
            .insert_resource(Inputs::input_map());
    }
}

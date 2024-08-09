use animation::AnimationControllerPlugin;
use bevy::prelude::*;

use self::{facing_direction::FacingDirectionPlugin, grounded::GroundedPlugin};

pub mod facing_direction;
pub mod grounded;
pub mod animation;

pub struct StateHandlerPlugin;

impl Plugin for StateHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((FacingDirectionPlugin, GroundedPlugin, AnimationControllerPlugin));
    }
}

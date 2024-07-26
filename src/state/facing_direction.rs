use bevy::prelude::*;

use crate::{
    input::{buffer::InputBuffer, inputs::Inputs},
    player::components::Player,
};

#[derive(Component)]
pub struct FacingDirection(f32);

impl FacingDirection {
    pub fn new() -> Self {
        FacingDirection(1.)
    }

    pub fn set(&mut self, dir: f32) {
        self.0 = dir.signum();
    }

    pub fn get(&self) -> f32 {
        self.0.signum()
    }
}

pub fn update_facing_direction_player(
    mut q_player: Query<(&mut FacingDirection, &InputBuffer), With<Player>>,
) {
    for (mut direction, buffer) in q_player.iter_mut() {
        let x_input = buffer.this_frame().x();

        if x_input.abs() > 0.1 && !buffer.blocked(Inputs::Directional) {
            direction.set(x_input);
        }
    }
}

pub struct FacingDirectionPlugin;

impl Plugin for FacingDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_facing_direction_player);
    }
}

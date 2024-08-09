use bevy::prelude::*;

use crate::{
    characters::demo_player::DemoPlayer,
    input::{buffer::InputBuffer, inputs::Inputs},
};

use super::animation::AnimationConfig;

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

    pub fn right(&self) -> bool {
        self.0.is_sign_positive()
    }

    pub fn left(&self) -> bool {
        self.0.is_sign_negative()
    }
}

pub fn update_facing_direction_player(
    mut q_player: Query<(&mut FacingDirection, &InputBuffer), With<DemoPlayer>>, // replace with Player trait
) {
    for (mut direction, buffer) in q_player.iter_mut() {
        let x_input = buffer.current_frame().x();

        if x_input.abs() > 0.1 && !buffer.blocked(Inputs::Directional) {
            direction.set(x_input);
        }
    }
}

pub fn update_sprite_direction(
    mut q_sprite: Query<(&Parent, &mut Sprite), With<AnimationConfig>>,
    q_direction: Query<&FacingDirection>,
) {
    for (interpreter, mut sprite) in &mut q_sprite {
        let Ok(facing) = q_direction.get(interpreter.get()) else {
            continue;
        };

        sprite.flip_x = !facing.right();
    }
}

pub struct FacingDirectionPlugin;

impl Plugin for FacingDirectionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (update_facing_direction_player, update_sprite_direction),
        );
    }
}

use bevy::app::Plugin;
use bevy::prelude::*;

use crate::input::inputs::Inputs;

use self::{
    crouch::CrouchBehavior, demo_slash::SlashingBehavior, jump::JumpBehavior,
    kick::KickingBehavior, shot::ShotBehavior, slide::SlidingBehavior, walk::WalkBehavior,
};

pub mod crouch;
pub mod demo_slash;
pub mod jump;
pub mod kick;
pub mod shot;
pub mod slide;
pub mod walk;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            KickingBehavior,
            SlidingBehavior,
            JumpBehavior,
            SlashingBehavior,
            CrouchBehavior,
            ShotBehavior,
            WalkBehavior,
        ));
    }
}

#[derive(Component)]
pub struct BehaviorInput<T: Component> {
    pub input: Inputs,
    pub behavior: T,
}

impl<T: Component> BehaviorInput<T> {
    pub fn new(input: Inputs, behavior: T) -> Self {
        Self { input, behavior }
    }

    pub fn get_mut(&mut self) -> (&mut T, Inputs) {
        (&mut self.behavior, self.input)
    }
}

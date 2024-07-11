use bevy::app::Plugin;

use self::{
    crouch::CrouchBehavior, 
    demo_slash::SlashingBehavior, 
    jump::JumpBehavior,
    kick::KickingBehavior, 
    // shot::ShotBehavior, 
    slide::SlidingBehavior,
};

pub mod crouch;
pub mod demo_slash;
pub mod jump;
pub mod kick;
// pub mod shot;
pub mod slide;

pub struct BehaviorPlugin;

impl Plugin for BehaviorPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugins((
            KickingBehavior,
            SlidingBehavior,
            JumpBehavior,
            SlashingBehavior,
            CrouchBehavior,
            // ShotBehavior,
        ));
    }
}

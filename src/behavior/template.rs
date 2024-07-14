use std::time::Duration;

use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

#[derive(Component)]
pub struct Behavior {
    // a refference to the current stage the behavior is in
    stage: Stage,
    // generic timer used for time-based stages
    stage_timer: Timer,
    // any other internal data would go below
}

pub enum Stage {
    // The stages your behavior can occupy
    // Normally you want a "Dormant" stage to denote when the behavior is inactive
}

impl Behavior {
    // return a default representation of your Behavior struct
    pub fn new() -> Self {}

    // a public interface function to allow for external stage changes
    // you should use this for any stage change logic
    // also, this is where you define stage specific internal state (eg. stage duration)
    pub fn set_stage(&mut self, next: Stage) {}

    // for any colliders you want to spawn, use a generic function like this
    pub fn spawn_collider(
        commands: &mut Commands,
        // any other data relevant to the attributes of your colliders
    ) -> Entity {
    }
}

// Here is where you define the logic for behaviors
pub fn behaviorname_player_behavior(
    shape_intersections: ShapeIntersections // This will be what you use for hitboxes
) {
    // you normally want to use a for loop for behavior system logic
    for (components ...) in _query_.iter_mut() {
        // update your timer externally from the stage logic
        // it helps with the borrow checker and reduces repeated code
        let timer_finished = state.stage_timer.tick(time.delta).finished();

        // while you can use _ as a match case to catch default cases
        // you should only do this if one of your cases uses a match conditional
        // make sure all your stages have logic before setting a default!
        match &state.stage {}
    }
}

// define a plugin to automate how it is integrated into the game
pub struct BehaviorNameBehavior;

impl Plugin for BehaviorNameBehavior {
    fn build(&self, app: &mut App) {}
}

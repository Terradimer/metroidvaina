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

    // it is highly recommended to simplify collision checks my passing it off to a generic function
    pub fn get_collision(&self, rapier_context: &RapierContext) -> Option<Entity> {
        // If you have a collider dependent on a specific stage, use this structure
        let collider_refs = match &self.stage {
            Stage::YourStageHere { collider } => colliders,
            _ => return None,
        };

        // Regardless of how you extract your collider data, you should extract
        // intersections like this:
        for collider in collider_refs.iter() {
            // remember that entity1 and entity2 will be random from intersections
            // meaning if you want a specific entity (like the collider hit)
            // you need to follow the pattern below
            for (entity1, entity2, _) in rapier_context
                .intersection_pairs_with(*collider)
                .filter(|(_, _, intersecting)| *intersecting)
            {
                if entity1 != *collider {
                    return Some(entity1);
                } else {
                    return Some(entity2);
                }
            }
        }
        None
    }
}

// Here is where you define the logic for behaviors
pub fn behaviorname_player_behavior() {
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

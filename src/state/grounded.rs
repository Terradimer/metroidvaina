use avian2d::collision::Collisions;
use bevy::prelude::*;

use crate::{player::components::Body, GameState};

#[derive(Component)]
pub struct Grounded {
    in_state: bool,
}

impl Grounded {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true;
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false;
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self { in_state: false }
    }
}

pub fn update_grounded(mut q_player: Query<(&mut Grounded, &Body)>, collisions: Res<Collisions>) {
    for (mut grounded, p_body) in q_player.iter_mut() {
        grounded.stop();

        for collision in collisions.collisions_with_entity(p_body.collider_ref) {
            for normal in collision.manifolds.iter().map(|manifold| {
                if collision.entity1 == p_body.collider_ref {
                    manifold.normal1
                } else {
                    manifold.normal2
                }
            }) {
                if normal.y < 0. {
                    grounded.start() // this already early returns
                }
            }
        }
    }
}

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_grounded.run_if(in_state(GameState::Playing)));
    }
}

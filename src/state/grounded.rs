use std::ops::Not;

use avian2d::prelude::{Collider, SpatialQuery};
use bevy::prelude::*;

use crate::{
    characters::Body,
    collision_groups::{CollisionGroup, ENVIRONMENT},
    GameState,
};

#[derive(Component)]
pub struct Grounded {
    in_state: bool,
}

impl Grounded {
    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self { in_state: false }
    }
}

pub fn update_grounded(
    mut q_grounded: Query<(&mut Grounded, &Transform, &Body)>,
    shape_intersections: SpatialQuery,
) {
    for (mut grounded, transform, body) in &mut q_grounded {
        grounded.in_state = shape_intersections
            .shape_intersections(
                &Collider::rectangle(body.width * 0.85, 0.1),
                Vec2::new(
                    transform.translation.x,
                    transform.translation.y - body.height / 2.,
                ),
                0.,
                CollisionGroup::filter(ENVIRONMENT),
            )
            .is_empty()
            .not();
    }
}

pub struct GroundedPlugin;

impl Plugin for GroundedPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_grounded.run_if(in_state(GameState::Playing)));
    }
}

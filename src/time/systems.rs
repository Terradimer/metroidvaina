use crate::time::resources::*;
use bevy::prelude::*;
use std::time::Duration;
use avian2d::prelude::*;

pub fn update_scaled_time(
    time: Res<Time>,
    mut time_scale: ResMut<ScaledTime>,
    mut time_physics: ResMut<Time<Physics>>,
) {
    time_scale.delta = Duration::from_secs_f32(time.delta_seconds() * time_scale.scale);
    // *time_physics = Time::new_with(Physics::variable((time_scale.delta_seconds() * time_scale.scale).max(0.000001).into()));
}

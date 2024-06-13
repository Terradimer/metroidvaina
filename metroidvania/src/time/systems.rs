use crate::time::resources::*;
use bevy::prelude::*;
use bevy_rapier2d::plugin::{RapierConfiguration, TimestepMode};
use std::time::Duration;

pub fn update_scaled_time(
    time: Res<Time>,
    mut time_scale: ResMut<ScaledTime>,
    mut rapier_config: ResMut<RapierConfiguration>,
) {
    time_scale.delta = Duration::from_secs_f32(time.delta_seconds() * time_scale.scale);
    rapier_config.timestep_mode = TimestepMode::Variable {
        max_dt: time.delta_seconds() * time_scale.scale,
        time_scale: time_scale.scale,
        substeps: 3,
    };
}

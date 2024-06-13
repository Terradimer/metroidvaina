use bevy::prelude::Resource;
use std::time::Duration;

#[derive(Resource)]
pub struct ScaledTime {
    pub scale: f32,
    pub delta: Duration,
    pub stored_scale: f32,
}

impl ScaledTime {
    pub fn default() -> Self {
        ScaledTime {
            scale: 1.,
            delta: Duration::from_secs_f32(0.),
            stored_scale: 1.,
        }
    }

    pub fn delta_seconds(&self) -> f32 {
        self.delta.as_secs_f32()
    }
}

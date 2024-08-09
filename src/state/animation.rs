use std::time::Duration;

use bevy::prelude::*;

#[derive(Component, Clone)]
pub struct AnimationConfig {
    pub end_index: usize,
    pub start_index: usize,
    pub frame_timer: Timer,
}

impl AnimationConfig {
    pub fn new(first: usize, last: usize, fps: u8) -> Self {
        Self {
            start_index: first,
            end_index: last,
            frame_timer: Self::timer_from_fps(fps),
        }
    }

    pub fn single_frame(index: usize) -> Self {
        Self {
            start_index: index,
            end_index: index,
            frame_timer: Timer::new(Duration::ZERO, TimerMode::Once),
        }
    }

    pub fn set_frames(&mut self, first: usize, last: usize) -> &mut Self {
        self.start_index = first;
        self.end_index = last;
        self
    }

    pub fn set_frames_from(&mut self, other: Self) -> &mut Self {
        self.start_index = other.start_index;
        self.end_index = other.end_index;
        self
    }

    pub fn set_fps(&mut self, fps: u8) {
        self.frame_timer
            .set_duration(Duration::from_secs_f32(1. / (fps as f32)));
    }

    fn timer_from_fps(fps: u8) -> Timer {
        Timer::new(Duration::from_secs_f32(1. / (fps as f32)), TimerMode::Once)
    }
}

// Systems

pub fn execute_animations(
    time: Res<Time>,
    mut animation_query: Query<(&mut AnimationConfig, &mut TextureAtlas)>,
) {
    for (mut config, mut atlas) in &mut animation_query {
        if config.frame_timer.tick(time.delta()).finished() {
            if atlas.index >= config.end_index || atlas.index < config.start_index {
                atlas.index = config.start_index;
            } else {
                atlas.index += 1;
            }
            config.frame_timer.reset();
        }
    }
}

pub struct AnimationControllerPlugin;

impl Plugin for AnimationControllerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, execute_animations);
    }
}

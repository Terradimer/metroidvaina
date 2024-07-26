//! Input frame and state representation for game input handling.
//!
//! This module provides the `InputFrame` struct for representing the state of all inputs
//! at a specific moment, and the `InputState` enum for representing the state of individual inputs.

use std::{
    mem::discriminant,
    time::{Duration, Instant},
};

use super::{directions::InputDirection, inputs::Inputs};
use bevy::prelude::*;

/// Represents the state of all inputs at a specific moment.
#[derive(Debug, Clone, Copy)]
pub struct InputFrame {
    pub(super) instant: Instant,
    pub(super) jump: InputState,
    pub(super) primary: InputState,
    pub(super) secondary: InputState,
    pub(super) special: InputState,
    pub(super) direction: InputDirection,
    pub(super) dir_raw: Vec2,
}

impl InputFrame {
    /// Creates a new `InputFrame` with all inputs in the released state.
    pub fn new() -> Self {
        Self {
            instant: Instant::now(),
            jump: InputState::Released,
            primary: InputState::Released,
            secondary: InputState::Released,
            special: InputState::Released,
            direction: InputDirection::Neutral,
            dir_raw: Vec2::ZERO,
        }
    }

    /// Checks if the given input was just pressed in this frame.
    pub fn just_pressed(&self, input: Inputs) -> bool {
        match input {
            Inputs::Jump => matches!(self.jump, InputState::JustPressed),
            Inputs::Primary => matches!(self.primary, InputState::JustPressed),
            Inputs::Secondary => matches!(self.secondary, InputState::JustPressed),
            Inputs::Special => matches!(self.special, InputState::JustPressed),
            Inputs::Pause | Inputs::Directional => false,
        }
    }

    /// Checks if the given input is being pressed in this frame.
    pub fn pressed(&self, input: Inputs) -> bool {
        match input {
            Inputs::Jump => matches!(self.jump, InputState::Pressed { .. }),
            Inputs::Primary => matches!(self.primary, InputState::Pressed { .. }),
            Inputs::Secondary => matches!(self.secondary, InputState::Pressed { .. }),
            Inputs::Special => matches!(self.special, InputState::Pressed { .. }),
            Inputs::Pause | Inputs::Directional => false,
        }
    }

    /// Checks if the given input was just released in this frame.
    pub fn just_released(&self, input: Inputs) -> bool {
        match input {
            Inputs::Jump => matches!(self.jump, InputState::JustReleased { .. }),
            Inputs::Primary => matches!(self.primary, InputState::JustReleased { .. }),
            Inputs::Secondary => matches!(self.secondary, InputState::JustReleased { .. }),
            Inputs::Special => matches!(self.special, InputState::JustReleased { .. }),
            Inputs::Pause | Inputs::Directional => false,
        }
    }

    /// Checks if the given input is released in this frame.
    pub fn released(&self, input: Inputs) -> bool {
        match input {
            Inputs::Jump => matches!(self.jump, InputState::Released),
            Inputs::Primary => matches!(self.primary, InputState::Released),
            Inputs::Secondary => matches!(self.secondary, InputState::Released),
            Inputs::Special => matches!(self.special, InputState::Released),
            Inputs::Pause | Inputs::Directional => false,
        }
    }

    /// Checks if the current direction matches the given direction.
    pub fn check_direction(&self, direction: InputDirection) -> bool {
        discriminant(&self.direction) == discriminant(&direction)
    }

    /// Returns the current input direction.
    pub fn direction(&self) -> InputDirection {
        self.direction
    }

    /// Returns the x-component of the raw directional input.
    pub fn x(&self) -> f32 {
        self.dir_raw.x
    }

    /// Returns the y-component of the raw directional input.
    pub fn y(&self) -> f32 {
        self.dir_raw.y
    }

    /// Returns the raw directional input as a `Vec2`.
    pub fn xy(&self) -> Vec2 {
        self.dir_raw
    }
}

/// Represents the state of an individual input.
#[derive(Debug, Clone, Copy)]
pub enum InputState {
    JustPressed,
    Pressed { duration: Duration },
    JustReleased { duration: Duration },
    Released,
}

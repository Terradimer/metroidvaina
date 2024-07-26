//! Input handling and mapping for game controls.
//!
//! This module provides the `Inputs` enum for representing different input actions,
//! along with related types and implementations for input mapping and state management.

use std::{mem::discriminant, time::Duration};

use bevy::{
    input::{gamepad::GamepadButtonType, keyboard::KeyCode},
    reflect::Reflect,
};
use leafwing_input_manager::{
    axislike::{DualAxis, VirtualDPad},
    input_map::InputMap,
    Actionlike,
};

use super::{
    blocker::{Blockable, Blocker},
    input_frame::{InputFrame, InputState},
    input_query::InputLike,
};

/// Represents the different input actions available in the game.
#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Inputs {
    Directional,
    Jump,
    Primary,
    Secondary,
    Special,
    Pause,
}

#[derive(Copy, Clone)]
pub struct InputType {
    input: Inputs,
    state: InputState,
}

impl InputLike for InputType {
    fn matches(&self, frame: &InputFrame) -> bool {
        match self.input {
            Inputs::Jump => discriminant(&frame.jump) == discriminant(&self.state),
            Inputs::Primary => discriminant(&frame.primary) == discriminant(&self.state),
            Inputs::Secondary => discriminant(&frame.secondary) == discriminant(&self.state),
            Inputs::Special => discriminant(&frame.special) == discriminant(&self.state),
            Inputs::Pause | Inputs::Directional => false,
        }
    }
}

impl Blockable for Inputs {
    fn to_blocker(&self) -> Blocker {
        match self {
            Inputs::Jump => Blocker::JUMP,
            Inputs::Primary => Blocker::PRIMARY,
            Inputs::Secondary => Blocker::SECONDARY,
            Inputs::Special => Blocker::SPECIAL,
            Inputs::Directional => Blocker::directions(),
            Inputs::Pause => Blocker::NONE,
        }
    }
}

impl Blockable for InputType {
    fn to_blocker(&self) -> Blocker {
        self.input.to_blocker()
    }
}

impl Inputs {
    // Creates and returns the default input map for the game.
    pub fn input_map() -> InputMap<Inputs> {
        let mut input_map = InputMap::default();

        input_map.insert(Self::Directional, VirtualDPad::wasd());
        input_map.insert(Self::Directional, DualAxis::left_stick());
        input_map.insert(Self::Directional, VirtualDPad::dpad());

        input_map.insert(Self::Jump, KeyCode::Space);
        input_map.insert(Self::Jump, GamepadButtonType::South);

        input_map.insert(Self::Pause, KeyCode::Escape);
        input_map.insert(Self::Pause, GamepadButtonType::Start);

        input_map.insert(Self::Primary, KeyCode::KeyV);

        input_map.insert(Self::Secondary, KeyCode::KeyB);

        input_map
    }

    /// Returns a vector of all available input actions.
    pub fn all_actions() -> Vec<Inputs> {
        vec![
            Self::Directional,
            Self::Jump,
            Self::Primary,
            Self::Secondary,
            Self::Special,
        ]
    }

    /// Creates an `InputType` representing a pressed state for this input.
    pub fn pressed(&self) -> InputType {
        InputType {
            input: *self,
            state: InputState::Pressed {
                duration: Duration::ZERO,
            },
        }
    }

    /// Creates an `InputType` representing a just released state for this input.
    pub fn just_released(&self) -> InputType {
        InputType {
            input: *self,
            state: InputState::JustReleased {
                duration: Duration::ZERO,
            },
        }
    }

    /// Creates an `InputType` representing a released state for this input.
    pub fn released(&self) -> InputType {
        InputType {
            input: *self,
            state: InputState::Released,
        }
    }
    
    /// Creates an `InputType` representing a just pressed state for this input.
    pub fn just_pressed(&self) -> InputType {
        InputType {
            input: *self,
            state: InputState::JustPressed,
        }
    }
}

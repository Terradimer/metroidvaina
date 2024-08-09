//! Directional input handling for game controls.
//!
//! This module provides the `InputDirection` enum for representing
//! cardinal and ordinal directions, along with related implementations
//! for input matching, blocking, and direction calculations.

use bevy::prelude::*;

use super::{
    blocker::{Blockable, Blocker},
    input_frame::InputFrame,
    input_query::InputLike,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum InputDirection {
    Up,
    UpRight,
    Right,
    DownRight,
    Down,
    DownLeft,
    Left,
    UpLeft,
    Neutral,
}

impl InputLike for InputDirection {
    fn matches(&self, frame: &InputFrame) -> bool {
        frame.direction == *self
    }
}

impl Blockable for InputDirection {
    fn to_blocker(&self) -> Blocker {
        match self {
            InputDirection::Up => Blocker::UP,
            InputDirection::UpRight => Blocker::UP_RIGHT,
            InputDirection::Right => Blocker::RIGHT,
            InputDirection::DownRight => Blocker::DOWN_RIGHT,
            InputDirection::Down => Blocker::DOWN,
            InputDirection::DownLeft => Blocker::DOWN_LEFT,
            InputDirection::Left => Blocker::LEFT,
            InputDirection::UpLeft => Blocker::UP_LEFT,
            InputDirection::Neutral => Blocker::NONE,
        }
    }
}

impl InputDirection {
    pub const ANY_DOWN: [Self; 3] = [Self::Down, Self::DownRight, Self::DownLeft];
    pub const ANY_UP: [Self; 3] = [Self::Up, Self::UpRight, Self::UpLeft];
    pub const ANY_RIGHT: [Self; 3] = [Self::Right, Self::UpRight, Self::DownRight];
    pub const ANY_LEFT: [Self; 3] = [Self::Left, Self::UpLeft, Self::DownLeft];

    fn from_u32(num: u32) -> Self {
        match num % 8 {
            0 => InputDirection::Up,
            1 => InputDirection::UpRight,
            2 => InputDirection::Right,
            3 => InputDirection::DownRight,
            4 => InputDirection::Down,
            5 => InputDirection::DownLeft,
            6 => InputDirection::Left,
            7 => InputDirection::UpLeft,
            _ => unreachable!(),
        }
    }

    /// Generates a vector of `InputDirection`s representing a clockwise roll from `self` to `to`.
    ///
    /// # Arguments
    ///
    /// * `to` - The target `InputDirection` to roll to.
    ///
    /// # Returns
    ///
    /// A `Vec<InputDirection>` containing the sequence of directions in the clockwise roll.
    pub fn roll_clockwise(&self, to: Self) -> Vec<InputDirection> {
        let end = to as u32 % 8;
        let start = *self as u32 % 8;
        let size = if start == end {
            9
        } else if start < end {
            end - start + 1
        } else {
            8 - start + end + 1
        };

        let mut result = Vec::with_capacity(size as usize);
        for i in 0..size {
            result.push(Self::from_u32(start + i));
        }
        result
    }

    /// Generates a vector of `InputDirection`s representing a counter-clockwise roll from `self` to `to`.
    ///
    /// # Arguments
    ///
    /// * `to` - The target `InputDirection` to roll to.
    ///
    /// # Returns
    ///
    /// A `Vec<InputDirection>` containing the sequence of directions in the counter-clockwise roll.
    pub fn roll_counter_clockwise(&self, to: Self) -> Vec<InputDirection> {
        let end = to as u32 % 8;
        let start = *self as u32 % 8;
        let size = if start == end {
            9
        } else if start > end {
            start - end + 1
        } else {
            8 + start - end + 1
        };

        let mut result = Vec::with_capacity(size as usize);
        for i in 0..size {
            result.push(Self::from_u32(8 + start - i));
        }
        result
    }

    /// Converts a raw 2D input vector to an `InputDirection`.
    ///
    /// # Arguments
    ///
    /// * `input` - A `Vec2` representing the raw input (e.g., from a joystick).
    ///
    /// # Returns
    ///
    /// The corresponding `InputDirection` based on the input vector's angle.
    pub fn from_raw(input: Vec2) -> Self {
        const DEADZONE: f32 = 0.2;

        if input.length() < DEADZONE {
            return Self::Neutral;
        }

        let (x, y) = (input.x, input.y);
        let degrees = y.atan2(x).to_degrees();

        match degrees {
            d if d >= 67.5 && d < 112.5 => InputDirection::Up,
            d if d >= 22.5 && d < 67.5 => InputDirection::UpRight,
            d if d >= -22.5 && d < 22.5 => InputDirection::Right,
            d if d >= -67.5 && d < -22.5 => InputDirection::DownRight,
            d if d >= -112.5 && d < -67.5 => InputDirection::Down,
            d if d >= -157.5 && d < -112.5 => InputDirection::DownLeft,
            d if d >= 157.5 || d < -157.5 => InputDirection::Left,
            d if d >= 112.5 && d < 157.5 => InputDirection::UpLeft,
            _ => unreachable!(),
        }
    }
}

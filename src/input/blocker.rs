//! Input blocking mechanism for game input handling.
//!
//! This module provides the `Blocker` struct for managing input blocking flags
//! and the `Blockable` trait for types that can be blocked.

use std::ops::{BitAnd, BitOr, Not};

/// Represents a set of input blocking flags.
///
/// Each flag corresponds to a specific input that can be blocked.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Blocker(u16);

impl Blocker {
    pub const NONE: Blocker = Blocker(0);
    pub const ALL: Blocker = Blocker(u16::MAX);

    pub const JUMP: Blocker = Blocker(1 << 0);
    pub const PRIMARY: Blocker = Blocker(1 << 1);
    pub const SECONDARY: Blocker = Blocker(1 << 2);
    pub const SPECIAL: Blocker = Blocker(1 << 3);
    pub const UP: Blocker = Blocker(1 << 4);
    pub const UP_RIGHT: Blocker = Blocker(1 << 5);
    pub const RIGHT: Blocker = Blocker(1 << 6);
    pub const DOWN_RIGHT: Blocker = Blocker(1 << 7);
    pub const DOWN: Blocker = Blocker(1 << 8);
    pub const DOWN_LEFT: Blocker = Blocker(1 << 9);
    pub const LEFT: Blocker = Blocker(1 << 10);
    pub const UP_LEFT: Blocker = Blocker(1 << 11);

    /// Returns the raw u16 value of the blocker.
    ///
    /// This method is intended for internal use within the module.
    pub(super) fn get(&self) -> u16 {
        self.0
    }

    /// Returns a `Blocker` with all directional input flags set.
    pub fn directions() -> Self {
        Blocker::UP
            | Blocker::UP_RIGHT
            | Blocker::RIGHT
            | Blocker::DOWN_RIGHT
            | Blocker::DOWN
            | Blocker::DOWN_LEFT
            | Blocker::LEFT
            | Blocker::UP_LEFT
    }

    /// Returns a `Blocker` with all non-directional input flags set.
    pub fn non_directional() -> Self {
        Blocker::JUMP | Blocker::PRIMARY | Blocker::SECONDARY | Blocker::SPECIAL
    }
}

/// A trait for types that can be converted to a `Blocker`.
pub trait Blockable {
    /// Converts the implementing type to a `Blocker`.
    fn to_blocker(&self) -> Blocker;
}

impl Blockable for Blocker {
    fn to_blocker(&self) -> Blocker {
        *self
    }
}

impl BitOr for Blocker {
    type Output = Self;

    /// Implements the `|` operator for combining `Blocker` instances.
    fn bitor(self, rhs: Self) -> Self::Output {
        Blocker(self.0 | rhs.0)
    }
}

impl BitAnd for Blocker {
    type Output = Self;

    /// Implements the `&` operator for intersecting `Blocker` instances.
    fn bitand(self, rhs: Self) -> Self::Output {
        Blocker(self.0 & rhs.0)
    }
}

impl Not for Blocker {
    type Output = Self;

    /// Implements the `!` operator for inverting a `Blocker` instance.
    fn not(self) -> Self::Output {
        Blocker(!self.0)
    }
}

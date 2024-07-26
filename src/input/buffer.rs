//! Input buffer management for game input handling.
//!
//! This module provides the `InputBuffer` component and related functionality
//! for managing and querying game input states over time.

use std::{collections::VecDeque, mem::discriminant};

use super::{
    blocker::{Blockable, Blocker},
    directions::InputDirection,
    input_frame::{InputFrame, InputState},
    input_query::{InputLike, InputQuery},
    inputs::Inputs,
};
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

/// Component for managing a buffer of input frames and current input state.
#[derive(Component)]
pub struct InputBuffer {
    buffer: VecDeque<InputFrame>,
    current_frame: InputFrame,
    blocker: Blocker,
}

impl InputBuffer {
    /// Creates a new, empty `InputBuffer`.
    pub fn new() -> Self {
        Self {
            buffer: VecDeque::new(),
            current_frame: InputFrame::new(),
            blocker: Blocker::NONE,
        }
    }

    /// Clears all stored input frames from the buffer.
    pub fn clear(&mut self) {
        self.buffer.clear()
    }

    /// Adds a new input frame to the buffer, removing the oldest if at capacity.
    fn add(&mut self, input: InputFrame) {
        if self.buffer.capacity() == self.buffer.len() {
            self.buffer.pop_front();
        }
        self.buffer.push_back(input);
    }

    /// Creates an `InputQuery` for querying the buffer's contents.
    pub fn query(&mut self) -> InputQuery {
        InputQuery {
            frames: self.buffer.make_contiguous().to_vec(),
            source: self,
        }
    }

    // Checks if the current frame matches the given input and is not blocked.
    pub fn is(&self, input: impl InputLike + Blockable) -> bool {
        input.matches(&self.current_frame) && !self.blocked(input.to_blocker())
    }

    /// Checks if the current frame matches any of the given inputs and is not blocked.
    pub fn any(&self, inputs: Vec<impl InputLike + Blockable>) -> bool {
        inputs
            .iter()
            .any(|input| input.matches(&self.current_frame) && !self.blocked(input.to_blocker()))
    }

    /// Returns the current input frame.
    pub fn this_frame(&self) -> InputFrame {
        self.current_frame
    }

    /// Applies the given blocker set to the current blocker.
    pub fn block(&mut self, blocker_set: Blocker) {
        self.blocker = self.blocker | blocker_set
    }

    /// Blocks all input.
    pub fn block_all(&mut self) {
        self.blocker = Blocker::ALL;
    }

    /// Clears all active blockers.   
    pub fn clear_blocker(&mut self) {
        self.blocker = Blocker::NONE;
    }
    
    /// Checks if the given blockable input is currently blocked.
    pub fn blocked(&self, blockable: impl Blockable) -> bool {
        (self.blocker & blockable.to_blocker()).get() != 0
    }
}

/// System for updating input buffers based on raw input state.
pub fn update_buffers(input_raw: Res<ActionState<Inputs>>, mut q_buffer: Query<&mut InputBuffer>) {
    for mut buffer in q_buffer.iter_mut() {
        let mut frame = InputFrame::new();
        let mut changed = false;

        for action in Inputs::all_actions() {
            if matches!(action, Inputs::Directional | Inputs::Pause) {
                continue;
            }

            let value = if input_raw.just_pressed(&action) {
                InputState::JustPressed
            } else if input_raw.pressed(&action) {
                InputState::Pressed {
                    duration: input_raw.current_duration(&action),
                }
            } else if input_raw.just_released(&action) {
                InputState::JustReleased {
                    duration: input_raw.previous_duration(&action),
                }
            } else {
                InputState::Released
            };

            match action {
                Inputs::Jump => {
                    if discriminant(&buffer.current_frame.jump) != discriminant(&value) {
                        changed = true
                    };
                    frame.jump = value
                }
                Inputs::Primary => {
                    if discriminant(&buffer.current_frame.primary) != discriminant(&value) {
                        changed = true
                    };
                    frame.primary = value
                }
                Inputs::Secondary => {
                    if discriminant(&buffer.current_frame.secondary) != discriminant(&value) {
                        changed = true
                    };
                    frame.secondary = value
                }
                Inputs::Special => {
                    if discriminant(&buffer.current_frame.special) != discriminant(&value) {
                        changed = true
                    };
                    frame.special = value
                }
                Inputs::Pause | Inputs::Directional => unreachable!(),
            }
        }

        let move_axis = match input_raw.clamped_axis_pair(&Inputs::Directional) {
            Some(data) => data.xy(),
            None => continue,
        };

        frame.direction = InputDirection::from_raw(move_axis);
        frame.dir_raw = move_axis;

        buffer.current_frame = frame;

        if changed {
            buffer.add(frame);
        }
    }
}

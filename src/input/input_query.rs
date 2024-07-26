//! This module provides functionality for querying and manipulating input events.
//!
//! It defines traits and structures for working with input frames, allowing
//! for filtering, sequencing, and time-based operations on input events.

use std::{
    collections::VecDeque,
    time::{Duration, Instant},
};

use super::{blocker::Blockable, buffer::InputBuffer, input_frame::InputFrame};

/// A trait for types that can be matched against an `InputFrame`.
pub trait InputLike {
    /// Checks if this input matches the given frame.
    ///
    /// # Arguments
    ///
    /// * `frame` - The `InputFrame` to match against.
    ///
    /// # Returns
    ///
    /// `true` if this input matches the frame, `false` otherwise.
    fn matches(&self, frame: &InputFrame) -> bool;
}

pub struct InputQuery<'a> {
    pub(super) frames: Vec<InputFrame>,
    pub(super) source: &'a mut InputBuffer,
}

impl<'a> InputQuery<'a> {
    /// Checks if the query is successful (is not empty)
    ///
    /// # Returns
    ///
    /// `true` if there are frames, `false` otherwise.    
    pub fn check(&self) -> bool {
        !self.frames.is_empty()
    }

    /// Checks if the query is successful (is not empty), and consumes the query if so
    ///
    /// # Returns
    ///
    /// `true` if frames were present and consumed, `false` otherwise.
    pub fn consume(&mut self) -> bool {
        let result = self.check();
        if result {
            self.source.clear();
        }
        result
    }

    /// Retains only the frames within the specified duration from now.
    ///
    /// # Arguments
    ///
    /// * `duration` - The time window to consider.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn within_timeframe(&mut self, duration: Duration) -> &mut Self {
        let now = Instant::now();
        self.frames
            .retain(|event| now.duration_since(event.instant) <= duration);
        self
    }

    /// Retains only the frames that contain the given input state.
    ///
    /// # Arguments
    ///
    /// * `filter` - The filter to apply.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn contains<T: InputLike + Blockable + Copy>(&mut self, filter: T) -> &mut Self {
        if self.source.blocked(filter) {
            self.frames.clear()
        }
        self.frames.retain(|frame| filter.matches(frame));
        self
    }

    /// Retains only the frames that match any of the given input states.
    ///
    /// # Arguments
    ///
    /// * `filter` - A vector of filters to apply.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn contains_any<T: InputLike + Blockable>(&mut self, filter: Vec<T>) -> &mut Self {
        let blocked_filter = filter
            .into_iter()
            .filter(|input| !self.source.blocked(input.to_blocker()))
            .collect::<Vec<T>>();

        self.frames
            .retain(|frame| blocked_filter.iter().any(|input| input.matches(frame)));
        self
    }

    /// Retains only the frames that match the given sequence of input states.
    ///
    /// # Arguments
    ///
    /// * `filter` - A vector of filters representing the sequence to match.
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn sequence<T: InputLike + Blockable>(&mut self, filter: Vec<T>) -> &mut Self {
        if filter.is_empty()
            || filter
                .iter()
                .any(|input| self.source.blocked(input.to_blocker()))
        {
            self.frames.clear();
            return self;
        }

        let mut result = Vec::new();
        let mut window = VecDeque::with_capacity(filter.len());
        let filter_len = filter.len();

        for frame in self.frames.iter() {
            window.push_back(frame);
            if window.len() > filter_len {
                window.pop_front();
            }

            if window.len() == filter_len && window.iter().zip(&filter).all(|(e, d)| d.matches(e)) {
                result.extend(window.drain(..));
            }
        }

        self.frames = result;
        self
    }

    /// Returns a reference to the most recent frame, if any.
    /// If the query has failed (is empty), it returns none.
    ///
    /// # Returns
    ///
    /// An `Option` containing a reference to the most recent `InputFrame`, or `None` if empty.
    pub fn check_recent(&self) -> Option<&InputFrame> {
        self.frames.first()
    }

    /// Consumes and returns the most recent frame, if any, clearing the source buffer on success.
    /// If the query has failed (is empty), it returns none.
    ///
    /// # Returns
    ///
    /// An `Option` containing the most recent `InputFrame`, or `None` if empty.
    pub fn consume_recent(&mut self) -> Option<InputFrame> {
        if let Some(&frame) = self.frames.first() {
            self.source.clear();
            return Some(frame);
        }
        None
    }

    /// Chains a new query on the input buffer, if the query being chained has "failed" (is empty)
    /// all query's chained will also fail.
    /// The new query will be be composed of the entire input buffer
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn and(&mut self) -> &mut Self {
        if self.check() {
            self.frames = self.source.query().frames;
        }
        self
    }
    
    /// Chains a new query on the input buffer, if the query being chained has "failed" (is empty)
    /// all query's chained will also fail.
    /// The new query will be composed of the input frames that occured AFTER the least recent remaining
    /// frame of the chained query.
    /// 
    /// This can be syntactically thought of as "chained_query" [happens] "before" "returned_query" 
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn before(&mut self) -> &mut Self {
        if let Some(instant) = self.frames.first().map(|frame| frame.instant) {
            self.frames = self
                .source
                .query()
                .frames
                .into_iter()
                .take_while(|frame| frame.instant > instant)
                .collect();
        }
        self
    }

    /// Chains a new query on the input buffer, if the query being chained has "failed" (is empty)
    /// all query's chained will also fail.
    /// The new query will be composed of the input frames that occured BEFORE the most recent remaining 
    /// frame of the chained query.
    /// 
    /// This can be syntactically thought of as "chained_query" [happens] "after" "returned_query" 
    ///
    /// # Returns
    ///
    /// A mutable reference to self for method chaining.
    pub fn after(&mut self) -> &mut Self {
        if let Some(instant) = self.frames.last().map(|frame| frame.instant) {
            self.frames = self
                .source
                .query()
                .frames
                .into_iter()
                .skip_while(|frame| frame.instant < instant)
                .collect();
        }
        self
    }
}

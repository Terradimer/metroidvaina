use bevy::prelude::*;

#[derive(Component)]
pub struct Grounded {
    in_state: bool,
}

impl Grounded {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self { in_state: false }
    }
}

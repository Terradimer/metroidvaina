use bevy::{prelude::*, utils::HashSet};

use super::Inputs;

#[derive(Resource)]
pub struct InputBlocker(HashSet<Inputs>);

#[allow(dead_code)]
impl InputBlocker {
    pub fn block(&mut self, input: Inputs) {
        self.0.insert(input);
    }

    pub fn block_many(&mut self, inputs: Vec<Inputs>) {
        for input in inputs {
            self.0.insert(input);
        }
    }

    pub fn check(&self, input: Inputs) -> bool {
        self.0.contains(&input)
    }

    pub fn clear(&mut self) {
        self.0.clear()
    }

    pub fn default() -> Self {
        Self(HashSet::new())
    }
}

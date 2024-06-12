use bevy::prelude::*;

#[derive(Default, PartialEq)]
pub enum FPSSetting {
    Uncapped,
    #[default]
    Auto,
    V15,
    V30,
    V60,
    V120,
    V240,
    V360,
    Custom,
}

#[derive(Resource)]
pub struct UiState {
    pub fps_setting: FPSSetting,
    pub fps_limit: f64,
}

impl Default for UiState {
    fn default() -> Self {
        Self { fps_setting: Default::default(), fps_limit: 60.0 }
    }
}
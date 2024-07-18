use bevy::prelude::*;
use leafwing_input_manager::{
    axislike::{DualAxis, VirtualDPad},
    input_map::InputMap,
    prelude::*,
    Actionlike,
};

use self::resources::InputBlocker;

// pub mod buffers;
pub mod resources;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug, Reflect)]
pub enum Inputs {
    Directional,
    Jump,
    Primary,
    Secondary,
    Special,
    Pause,
}

impl Inputs {
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

    pub fn all_actions() -> Vec<Inputs> {
        vec![
            Self::Directional,
            Self::Jump,
            Self::Primary,
            Self::Secondary,
            Self::Special,
        ]
    }
}

pub struct InputHandlerPlugin;

impl Plugin for InputHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(InputManagerPlugin::<Inputs>::default())
            .init_resource::<ActionState<Inputs>>()
            .insert_resource(Inputs::input_map())
            .insert_resource(InputBlocker::default());
    }
}

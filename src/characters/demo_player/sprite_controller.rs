
use avian2d::{parry::na::ComplexField, prelude::*};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;

use crate::{
    characters::demo_player::DemoPlayer,
    state::{animation::AnimationConfig, grounded::Grounded},
};
use crate::behavior::{
        jump::Jump,
        walk::{self, Walk},
    };

#[derive(AssetCollection, Resource)]
pub struct DemoPlayerSprites {
    #[asset(texture_atlas_layout(tile_size_x = 96, tile_size_y = 84, columns = 6, rows = 5))]
    pub(crate) layout: Handle<TextureAtlasLayout>,

    #[asset(image(sampler = nearest))]
    #[asset(path = "demoplayer/demo_player_sheet.png")]
    pub(crate) texture: Handle<Image>,
}

impl DemoPlayerSprites {
    pub fn idle_anim() -> AnimationConfig {
        AnimationConfig::new(6, 12, 15)
    }

    pub fn walk_anim() -> AnimationConfig {
        AnimationConfig::new(22, 29, 1)
    }

    pub fn slowdown_anim_1() -> AnimationConfig {
        AnimationConfig::new(13, 13, 1)
    }

    pub fn slowdown_anim_2() -> AnimationConfig {
        AnimationConfig::new(14, 14, 1)
    }
}

#[derive(Component)]
pub struct DemoPlayerAnimationInterpreter {
    config: Entity,
}

impl DemoPlayerAnimationInterpreter {
    pub fn new(config: Entity) -> Self {
        Self { config }
    }
}

pub fn interpret_sprite(
    q_interpreter: Query<(Entity, &DemoPlayerAnimationInterpreter)>,
    mut interpreters: ParamSet<(
        Query<(&Walk, &Grounded, &LinearVelocity)>, // Walking (0)
        Query<(&Jump, &Grounded, &LinearVelocity)>, // Jumping (1)
    )>,
    mut q_config: Query<&mut AnimationConfig>,
) {
    for (entity, interpreter) in &q_interpreter {
        let mut config = match q_config.get_mut(interpreter.config) {
            Ok(config) => config,
            Err(e) => {
                warn!(
                    "Error occured in idle animation interpretation for Demo Player {entity}: {e}"
                );
                continue;
            }
        };

        if let Ok((walk, grounded, vel)) = interpreters.p0().get(entity) {
            if grounded.check() {
                match walk.stage() {
                    walk::Stage::Active => {
                        config.set_frames_from(DemoPlayerSprites::walk_anim());
                        continue;
                    }
                    walk::Stage::Slowing => {
                        if vel.x.abs() > walk.max_speed() * 0.15 {
                            config.set_frames_from(DemoPlayerSprites::slowdown_anim_1());
                        } else {
                            config.set_frames_from(DemoPlayerSprites::slowdown_anim_2());
                        }
                        continue;
                    }
                    walk::Stage::Dormant => {}
                }
            }
        }
        
        if let Ok((jump, grounded, vel)) = interpreters.p1().get(entity) {
            if !grounded.check() {
                match jump.stage() {
                    crate::behavior::jump::Stage::Dormant => {},
                    crate::behavior::jump::Stage::Active => {},
                }
            }
        }

        config.set_frames_from(DemoPlayerSprites::idle_anim());
    }
}

pub fn interpret_jump(q_player: Query<(&Jump, &Grounded, &LinearVelocity), With<DemoPlayer>>) {
    todo!();
}

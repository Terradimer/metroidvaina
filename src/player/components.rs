use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Body {
    pub height: f32,
    pub width: f32,
    pub collider_ref: Entity,
}

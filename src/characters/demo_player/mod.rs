use avian2d::prelude::*;
use bevy::prelude::*;
use bevy_asset_loader::loading_state::{
    config::{ConfigureLoadingState, LoadingStateConfig},
    LoadingStateAppExt,
};
use sprite_controller::{interpret_sprite, DemoPlayerAnimationInterpreter, DemoPlayerSprites};

use crate::{
    behavior::{
        crouch::Crouch, demo_slash::DemoSlash, jump::Jump, kick::Kick, shot::Shot, slide::Slide,
        walk::Walk, BehaviorInput,
    },
    characters::Body,
    collision_groups::{CollisionGroup, PLAYER},
    input::{buffer::InputBuffer, inputs::Inputs},
    state::{facing_direction::FacingDirection, grounded::Grounded},
    GameState,
};

mod sprite_controller;

#[derive(Component)]
pub struct DemoPlayer;

pub fn spawn_demo_player(mut commands: Commands, demo_player_sprites: Res<DemoPlayerSprites>) {
    let player_sprite = commands
        .spawn((
            SpriteBundle {
                texture: demo_player_sprites.texture.clone(),
                transform: Transform {
                    scale: Vec3::new(2.5, 2.5, 1.),
                    translation: Vec3::Y * 55.,
                    ..default()
                },
                ..default()
            },
            TextureAtlas {
                layout: demo_player_sprites.layout.clone(),
                index: 0,
            },
            DemoPlayerSprites::idle_anim(),
        ))
        .id();

    // Load the base for the player behavior
    let height = 100.;
    let width = 50.;

    let collider_ref = commands
        .spawn((
            SpatialBundle::default(),
            CollisionGroup::COLLIDER,
            Collider::rectangle(width, height),
            Name::new("PlayerCollider"),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
        ))
        .id();

    let hurtbox_ref = commands
        .spawn((
            SpatialBundle::default(),
            Sensor,
            CollisionGroup::hurtbox(PLAYER),
            Collider::rectangle(width, height),
            Name::new("PlayerHurtbox"),
        ))
        .id();

    let player_body = Body {
        height,
        width,
        collider_ref,
    };

    commands
        .spawn((
            SpatialBundle::default(),
            DemoPlayer,
            Grounded::new(),
            player_body,
            FacingDirection::new(),
            InputBuffer::new(),
        ))
        .insert((
            RigidBody::Dynamic,
            GravityScale(1.),
            SweptCcd::default(),
            Friction::new(0.),
            DemoPlayerAnimationInterpreter::new(player_sprite),
            Restitution::ZERO.with_combine_rule(CoefficientCombine::Min),
            LinearVelocity::default(),
            LockedAxes::ROTATION_LOCKED,
            Name::new("Player"),
        ))
        .insert((
            Crouch::new(),
            Walk::new(4.3, 300., 3.),
            BehaviorInput::<DemoSlash>::new(Inputs::Primary, DemoSlash::new()),
            BehaviorInput::<Shot>::new(Inputs::Secondary, Shot::new()),
            Slide::new(500.),
            Jump::new(500.),
            Kick::new(2200.),
        ))
        .add_child(collider_ref)
        .add_child(player_sprite)
        .add_child(hurtbox_ref);
}

pub struct DemoPlayerPlugin;

impl Plugin for DemoPlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn_demo_player)
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading).load_collection::<DemoPlayerSprites>(),
            )
            .add_systems(
                Update,
                interpret_sprite.run_if(in_state(GameState::Playing)),
            );
    }
}

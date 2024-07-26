use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};

use crate::{
    behavior::{
        crouch::Crouch, demo_slash::DemoSlash, jump::Jump, kick::Kick, shot::Shot, slide::Slide,
        walk::Walk, BehaviorInput,
    },
    collision_groups::{CollisionGroup, PLAYER},
    input::{buffer::InputBuffer, inputs::Inputs}, state::{facing_direction::FacingDirection, grounded::Grounded},
};

use super::components::*;

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
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
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.0))),
                material: materials.add(Color::srgb(0., 0., 1.)),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Player,
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
        .add_child(hurtbox_ref);
}

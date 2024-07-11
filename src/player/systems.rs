use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{dynamics::RigidBody, prelude::*};
use leafwing_input_manager::action_state::ActionState;

use crate::{
    behavior::{
        crouch::Crouch, demo_slash::DemoSlash, jump::Jumping, kick::Kick, shot::Shot, slide::Slide,
    },
    collision_groups::Groups,
    input::{resources::InputBlocker, Inputs},
    macros::query_guard,
    time::resources::ScaledTime,
};

use super::{components::*, PLAYER_ACCELERATION_FACTOR, PLAYER_MAX_SPEED, PLAYER_SLOWING_FACTOR};

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
            Groups::collision(),
            Collider::cuboid(width / 2., height / 2.),
        ))
        .id();

    let hurtbox_ref = commands
        .spawn((
            SpatialBundle::default(),
            Groups::hurtbox(Groups::PLAYER),
            Collider::cuboid(width / 2., height / 2.),
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
                material: materials.add(Color::BLUE),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Player,
            Grounded::new(),
            player_body,
            FacingDirection::new(),
        ))
        .insert((
            RigidBody::Dynamic,
            GravityScale(1.),
            Friction::new(0.),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
        ))
        .insert((
            Crouch::new(),
            DemoSlash::new(),
            Slide::new(700.),
            Jumping::new(600.),
            Kick::new(),
            Shot::new(),
        ))
        .add_child(collider_ref)
        .add_child(hurtbox_ref);
}

pub fn horizontal_movement(
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
    mut q_player: Query<(&mut Velocity, &Crouch), With<Player>>,
    time: Res<ScaledTime>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let (mut velocity, crouching) = query_guard!(q_player.get_single_mut());

    let vel = &mut velocity.linvel;

    if !(move_axis.x.abs() > 0.2)
        || vel.x.signum() * move_axis.x.signum() < 0.
        || input_blocker.check(Inputs::Directional)
    {
        vel.x -= vel.x * PLAYER_SLOWING_FACTOR * time.delta_seconds();
    }

    if crouching.check() || input_blocker.check(Inputs::Directional) {
        return;
    }

    vel.x = (vel.x
        + move_axis.x * PLAYER_MAX_SPEED * PLAYER_ACCELERATION_FACTOR * time.delta_seconds())
    .clamp(-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);
}

pub fn update_facing_direction(
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
    mut q_player: Query<&mut FacingDirection, With<Player>>,
) {
    let mut direction = query_guard!(q_player.get_single_mut());

    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    if move_axis.x.abs() > 0.1 && !input_blocker.check(Inputs::Directional) {
        direction.set(move_axis.x);
    }
}

pub fn update_contact(
    mut q_player: Query<(&mut Grounded, &Body), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (mut grounded, p_body) = query_guard!(q_player.get_single_mut());

    grounded.stop();

    for contact_pair in rapier_context
        .contact_pairs_with(p_body.collider_ref)
        .filter(|contact_pair| contact_pair.has_any_active_contacts())
    {
        for normal in contact_pair.manifolds().map(|manifold| manifold.normal()) {
            if normal.y < 0. {
                grounded.start() // this already early returns
            }
        }
    }
}

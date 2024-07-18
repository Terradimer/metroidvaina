use avian2d::prelude::*;
use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use leafwing_input_manager::action_state::ActionState;

use crate::behavior::{jump::Jump, kick::Kick};
use crate::{behavior::crouch::Crouch, collision_groups::CollisionGroup};
use crate::{behavior::slide::Slide, collision_groups::PLAYER};
use crate::{
    behavior::{
        // crouch::Crouch,
        demo_slash::DemoSlash,
        // jump::Jumping,
        // kick::Kick,
        // shot::Shot,
        // slide::Slide,
    },
    input::{resources::InputBlocker, Inputs},
    macros::query_guard,
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
            DemoSlash::new(),
            Slide::new(700.),
            Jump::new(600.),
            Kick::new(2200.),
            // Shot::new(),
        ))
        .add_child(collider_ref)
        .add_child(hurtbox_ref);
}

pub fn horizontal_movement(
    input: Res<ActionState<Inputs>>,
    input_blocker: Res<InputBlocker>,
    mut q_player: Query<(&mut LinearVelocity, &Crouch), With<Player>>,
    time: Res<Time>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let (mut vel, crouching) = query_guard!(q_player.get_single_mut());

    if move_axis.x.abs() <= 0.2
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
    collisions: Res<Collisions>,
) {
    // dbg!(collisions.iter().map(|_| 1).sum::<i32>());
    let (mut grounded, p_body) = query_guard!(q_player.get_single_mut());

    grounded.stop();

    for collision in collisions.collisions_with_entity(p_body.collider_ref) {
        for normal in collision.manifolds.iter().map(|manifold| {
            if collision.entity1 == p_body.collider_ref {
                manifold.normal1
            } else {
                manifold.normal2
            }
        }) {
            if normal.y < 0. {
                return grounded.start();
            }
        }
    }
}

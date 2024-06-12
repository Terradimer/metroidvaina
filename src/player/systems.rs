use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{dynamics::RigidBody, prelude::*};
use leafwing_input_manager::action_state::ActionState;

use crate::{input::Inputs, macros::query_guard, time::resources::ScaledTime};

use super::{
    components::*, state::*, PLAYER_ACCELERATION_FACTOR, PLAYER_MAX_SPEED, PLAYER_SLOWING_FACTOR,
};

pub fn startup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands
        .spawn((
            MaterialMesh2dBundle {
                mesh: Mesh2dHandle(meshes.add(Rectangle::new(50., 100.0))),
                material: materials.add(Color::BLUE),
                transform: Transform::from_translation(Vec3::ZERO),
                ..default()
            },
            Player,
            Kicking::new(),
            Crouching::new(),
            Grounded::new(),
            FacingDirection::new(),
            Jumping::new(2),
            InputFreeze::new(),
        ))
        .insert((
            RigidBody::Dynamic,
            // Collider::cuboid(25., 50.),
            // Sensor,
            GravityScale(1.),
            Friction::new(0.),
            Velocity::default(),
            LockedAxes::ROTATION_LOCKED,
            ColliderMassProperties::Mass(1.),
        ))
        .with_children(|parent| {
            parent.spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0., 25., 0.)),
                UpperCollider,
                CollisionGroups {
                    memberships: Group::from_bits_retain(2),
                    filters: Group::from_bits_retain(1),
                },
                Collider::cuboid(25., 25.),
            ));
            parent.spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0., -25., 0.)),
                LowerCollider,
                CollisionGroups {
                    memberships: Group::from_bits_retain(2),
                    filters: Group::from_bits_retain(1),
                },
                // Sensor,
                Collider::cuboid(25., 25.),
            ));
            parent.spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0., 25., 0.)),
                StuckCheck,
                Sensor,
                CollisionGroups {
                    memberships: Group::from_bits_retain(2),
                    filters: Group::from_bits_retain(1),
                },
                Collider::cuboid(15., 25.),
            ));
        });
}

pub fn horizontal_movement(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<(&mut Velocity, &Crouching, &Kicking, &InputFreeze), With<Player>>,
    time: Res<ScaledTime>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let (mut velocity, crouching, kicking, input_freeze) = query_guard!(q_player.get_single_mut());

    if kicking.check() || !input_freeze.check() || crouching.stuck {
        return;
    }

    let vel = &mut velocity.linvel;

    if !(move_axis.x.abs() > 0.2) || vel.x.signum() * move_axis.x.signum() < 0. {
        vel.x -= vel.x * PLAYER_SLOWING_FACTOR * time.delta_seconds();
    }

    vel.x = (vel.x
        + move_axis.x * PLAYER_MAX_SPEED * PLAYER_ACCELERATION_FACTOR * time.delta_seconds())
    .clamp(-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);
}

pub fn jump(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<
        (
            &Velocity,
            &mut Kicking,
            &mut Jumping,
            &Grounded,
            &InputFreeze,
        ),
        With<Player>,
    >,
) {
    let (velocity, mut kicking, mut jumping, grounded, input_freeze) =
        query_guard!(q_player.get_single_mut());

    if grounded.check() {
        jumping.refill_jumps();
    }

    if !input_freeze.check() {
        return;
    }

    if input.just_pressed(&Inputs::Jump) && jumping.can_jump() && !kicking.is_changed() {
        jumping.start();
        kicking.stop();
        return;
    }

    if jumping.check() && (!input.pressed(&Inputs::Jump) || velocity.linvel.y < 0.) {
        jumping.stop();
    }
}

pub fn update_facing_direction(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<&mut FacingDirection, With<Player>>,
) {
    let mut direction = query_guard!(q_player.get_single_mut());

    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    if move_axis.x.abs() > 0.1 {
        direction.set(move_axis.x);
    }
}

pub fn crouching(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<
        (
            &mut Velocity,
            &FacingDirection,
            &Grounded,
            &mut Crouching,
            &mut InputFreeze,
        ),
        With<Player>,
    >,
) {
    let (mut velocity, direction, grounded, mut crouching, mut input_freeze) =
        query_guard!(q_player.get_single_mut());

    let input_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    if grounded.check() && input_freeze.check() {
        if input_axis.y < 0. && input_axis.x.abs() <= 0.2 {
            crouching.start();
        } else {
            crouching.stop();
        }
    }

    // sliding
    if (crouching.check() || crouching.stuck)
        && input.just_pressed(&Inputs::Jump)
        && input_freeze.check()
    {
        input_freeze.set(0.6);
        velocity.linvel.x = 500. * direction.get();
    }
}

pub fn kicking(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<(&mut Kicking, &Jumping, &Grounded), With<Player>>,
) {
    let (mut s_kicking, s_jumping, s_grounded) = query_guard!(q_player.get_single_mut());

    let y_input = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.y(),
        None => return,
    };

    if input.just_pressed(&Inputs::Jump)
        && !(s_kicking.check() || s_grounded.check())
        && s_jumping.has_air_jumped()
        && y_input < 0.
    {
        s_kicking.start();
    }

    if s_kicking.check() && s_grounded.check() {
        s_kicking.stop();
    }
}

pub fn update_contact(
    mut q_player: Query<&mut Grounded, With<Player>>,
    q_collider: Query<Entity, With<LowerCollider>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_entity, mut s_grounded) =
        query_guard!(q_collider.get_single(), q_player.get_single_mut());

    s_grounded.stop();

    for contact_pair in rapier_context
        .contact_pairs_with(p_entity)
        .filter(|contact_pair| contact_pair.has_any_active_contacts())
    {
        for normal in contact_pair.manifolds().map(|manifold| manifold.normal()) {
            if normal.y < 0. {
                s_grounded.start() // early return out
            }
        }
    }
}

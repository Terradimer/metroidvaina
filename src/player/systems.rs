use bevy::{
    prelude::*,
    sprite::{MaterialMesh2dBundle, Mesh2dHandle},
};
use bevy_rapier2d::{dynamics::RigidBody, prelude::*};
use leafwing_input_manager::action_state::ActionState;

use crate::{input::Inputs, macros::query_guard, time::resources::ScaledTime};

use super::{components::*, state::*, PLAYER_MAX_SPEED};

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
            Grounded::new(),
            Jumping::new(2),
        ))
        .insert((
            RigidBody::Dynamic,
            Collider::cuboid(25., 50.),
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
                Collider::cuboid(25., 25.),
            ));
            parent.spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0., -25., 0.)),
                LowerCollider,
                Collider::cuboid(25., 25.),
            ));
        });
}

pub fn horizontal_movement(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<(&mut Velocity, &Kicking), With<Player>>,
    time: Res<ScaledTime>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    let (mut velocity, kicking) = query_guard!(q_player.get_single_mut());

    if kicking.check() {
        return;
    }

    let vel = &mut velocity.linvel;

    if !(move_axis.x.abs() > 0.) || vel.x.signum() * move_axis.x.signum() < 0. {
        vel.x -= vel.x * 3.25 * time.delta_seconds();
    }

    vel.x = (vel.x + move_axis.x * PLAYER_MAX_SPEED * 3. * time.delta_seconds())
        .clamp(-PLAYER_MAX_SPEED, PLAYER_MAX_SPEED);
}

pub fn jump(
    input: Res<ActionState<Inputs>>,
    mut q_player: Query<(&Velocity, &mut Kicking, &mut Jumping, &Grounded), With<Player>>,
) {
    let (velocity, mut kicking, mut jumping, grounded) = query_guard!(q_player.get_single_mut());

    if grounded.check() {
        jumping.refill_jumps();
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
    mut q_player: Query<(Entity, &mut Grounded), With<Player>>,
    rapier_context: Res<RapierContext>,
) {
    let (p_entity, mut s_grounded) = query_guard!(q_player.get_single_mut());

    s_grounded.stop();

    for contact_pair in rapier_context
        .contact_pairs_with(p_entity)
        .filter(|contact_pair| contact_pair.has_any_active_contacts())
    {
        for normal in contact_pair.manifolds().map(|manifold| manifold.normal()) {
            if normal.y > 0. {
                s_grounded.start() // early return out
            }
        }
    }
}

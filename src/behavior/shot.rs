use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    collision_groups::*,
    input::{resources::InputBlocker, Inputs},
    player::components::{FacingDirection, Grounded, Player},
    shape_intersections::ShapeIntersections,
};

use super::BehaviorInput;

#[derive(Component)]
pub struct Shot {
    stage: Stage,
    stage_timer: Timer,
}

#[derive(Component)]
pub struct Projectile {
    direction: f32,
}

pub enum Stage {
    Dormant,
    Stall,
}

impl Shot {
    pub fn new() -> Self {
        Self {
            stage: Stage::Dormant,
            stage_timer: Timer::from_seconds(0.5, TimerMode::Once),
        }
    }

    pub fn set_stage(&mut self, stage: Stage) {
        self.stage = stage;
        self.stage_timer.reset();
    }

    pub fn spawn_projectile(commands: &mut Commands, origin: Vec3, direction: f32) {
        commands.spawn((
            SpatialBundle::from_transform(Transform::from_translation(origin)),
            Projectile { direction },
            Name::new("Bullet"),
        ));
    }
}

pub fn projectile_behavior(
    mut commands: Commands,
    mut q_bullet: Query<(Entity, &mut Transform, &Projectile)>,
    time: Res<Time>,
    mut shape_intersections: ShapeIntersections,
) {
    for (collider, mut transform, projectile) in q_bullet.iter_mut() {
        transform.translation.x += 500. * time.delta_seconds() * projectile.direction;

        // I use a rectangle collider because circle colliders dont render in debug for some reason
        if let Some(other) = shape_intersections
            .shape_intersections(
                &Collider::rectangle(15., 15.),
                transform.translation.xy(),
                0.,
                CollisionGroup::filter(ENEMY | ENVIRONMENT),
            )
            .first()
        {
            // Since we dont have "hp" yet, we just despawn the bullet
            println!("Shot hit: {other:?}");
            commands.entity(collider).despawn_recursive();
        }
    }
}

pub fn shot_player_behavior(
    mut commands: Commands,
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    time: Res<Time>,
    mut q_player: Query<
        (
            &Transform,
            &mut LinearVelocity,
            &FacingDirection,
            &mut BehaviorInput<Shot>,
            &Grounded,
        ),
        With<Player>,
    >,
) {
    for (transform, mut velocity, direction, mut behavior_input, grounded) in q_player.iter_mut() {
        let (behavior, inputs) = behavior_input.get_mut();
        let timer_finished = behavior.stage_timer.tick(time.delta()).finished();

        match &behavior.stage {
            Stage::Dormant if input.just_pressed(&inputs) && !input_blocker.check(inputs) => {
                behavior.set_stage(Stage::Stall);
                Shot::spawn_projectile(&mut commands, transform.translation, direction.get());

                if grounded.check() {
                    input_blocker.block_many(Inputs::all_actions());
                    velocity.x = 0.;
                } else {
                    input_blocker.block_many(Inputs::non_directional());
                }
            }
            Stage::Stall if timer_finished => {
                behavior.set_stage(Stage::Dormant);
                input_blocker.clear();
            }
            _ => {}
        }
    }
}

pub struct ShotBehavior;

impl Plugin for ShotBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (shot_player_behavior, projectile_behavior));
    }
}

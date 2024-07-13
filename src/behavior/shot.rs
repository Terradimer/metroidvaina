use bevy::prelude::*;
use avian2d::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::{
    collision_groups::Groups,
    input::{resources::InputBlocker, Inputs},
    player::components::{FacingDirection, Player},
    time::resources::ScaledTime,
};

#[derive(Component)]
pub struct Shot {
    stage: Stage,
    stage_timer: Timer,
}

#[derive(Component)]
pub struct Projectile;

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
            Projectile,
            Collider::circle(6.),
            Sensor,
            Groups::hitbox(Groups::ENEMY),
            Name::new("ShotSensor"),
            // Ccd::enabled(),
        ));
    }
}

pub fn projectile_behavior(
    mut commands: Commands,
    mut q_bullet: Query<(Entity, &mut Transform), With<Projectile>>,
    time: Res<ScaledTime>,
    q_colliding_entities: Query<&CollidingEntities>,
) {
    for (collider, mut tranform) in q_bullet.iter() {
        // tranform.translation.x += 300. * time.delta_seconds();
        
        for (entity1, entity2, intersects) in rapier_context.intersection_pairs_with(collider)
        // .filter(|(_, _, intersecting)| *intersecting)
        {
            println!("{entity1:?} {entity2:?} {intersects}");
            // add another query and check if the hit entity has a health component
            // if not, proceed as normal
            // Since we dont have "hp" yet, we dont use either entity collision and just despawn the bullet
            // commands.entity(collider).despawn();
        }
    }
}

pub fn shot_player_behavior(
    mut commands: Commands,
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    time: Res<ScaledTime>,
    mut q_player: Query<(&Transform, &FacingDirection, &mut Shot), With<Player>>,
) {
    for (transform, direction, mut state) in q_player.iter_mut() {
        let timer_finished = state.stage_timer.tick(time.delta).finished();

        match &state.stage {
            Stage::Dormant
                if input.just_pressed(&Inputs::Secondary)
                    && !input_blocker.check(Inputs::Secondary) =>
            {
                state.set_stage(Stage::Stall);
                Shot::spawn_projectile(&mut commands, transform.translation, direction.get());
                input_blocker.block_many(Inputs::all_actions());
            }
            Stage::Stall if timer_finished => {
                state.set_stage(Stage::Dormant);
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

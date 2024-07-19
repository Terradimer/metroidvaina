use std::time::Duration;

use avian2d::prelude::*;
use bevy::prelude::*;
use leafwing_input_manager::action_state::ActionState;

use crate::collision_groups::ENEMY;
use crate::shape_intersections::ShapeIntersections;
use crate::{
    collision_groups::*,
    input::{resources::InputBlocker, Inputs},
    player::components::{FacingDirection, Grounded, Player},
};

use super::BehaviorInput;

#[derive(Component)]
pub struct DemoSlash {
    stage: Stage,
    stage_timer: Timer,
    has_hit: bool,
}

pub enum Stage {
    Dormant,
    Windup,
    Active,
    Settle,
}

impl DemoSlash {
    pub fn new() -> Self {
        Self {
            has_hit: false,
            stage: Stage::Dormant,
            stage_timer: Timer::from_seconds(0., TimerMode::Once),
        }
    }

    pub fn set_stage(&mut self, next: Stage) {
        match next {
            Stage::Windup | Stage::Settle => {
                self.stage_timer.set_duration(Duration::from_secs_f32(0.1));
            }
            Stage::Active { .. } => {
                self.has_hit = false;
                self.stage_timer.set_duration(Duration::from_secs_f32(0.3));
            }
            _ => {}
        }
        self.stage = next;
        self.stage_timer.reset();
    }
}

pub fn demo_slash_player_behavior(
    input: Res<ActionState<Inputs>>,
    mut input_blocker: ResMut<InputBlocker>,
    mut q_state: Query<
        (
            &mut LinearVelocity,
            &mut BehaviorInput<DemoSlash>,
            &Grounded,
            &FacingDirection,
            &Transform,
        ),
        With<Player>,
    >,
    time: Res<Time>,
    mut shape_intersections: ShapeIntersections,
) {
    for (mut vel, mut behavior_input, grounded, direction, transform) in q_state.iter_mut() {
        let (behavior, inputs) = behavior_input.get_mut();
        let timer_finished = behavior.stage_timer.tick(time.delta()).finished();

        match &behavior.stage {
            Stage::Dormant if input.just_pressed(&inputs) && !input_blocker.check(inputs) => {
                behavior.set_stage(Stage::Windup);
                input_blocker.block_many(Inputs::non_directional());

                if grounded.check() {
                    input_blocker.block_many(Inputs::all_actions());
                    vel.x = 0.;
                }
            }
            Stage::Windup if timer_finished => {
                behavior.set_stage(Stage::Active);
            }
            Stage::Active if timer_finished => {
                behavior.set_stage(Stage::Settle);
                input_blocker.clear();
            }
            Stage::Active if !behavior.has_hit => {
                let collider_size: f32 = 25.;
                let slash_size_side = collider_size / 2.25;
                for i in 0..2 {
                    if let Some(other) = shape_intersections
                        .shape_intersections(
                            &Collider::rectangle(slash_size_side, slash_size_side),
                            transform.translation.xy()
                                + Vec2::new(
                                    collider_size * 2. * direction.get(),
                                    collider_size
                                        + slash_size_side
                                        + 2. * (-collider_size - slash_size_side) * i as f32,
                                ),
                            0.,
                            CollisionGroup::filter(ENEMY),
                        )
                        .first()
                    {
                        println!("Slashed: {other:?}",);
                        behavior.has_hit = true;
                        return;
                    }
                }

                if let Some(other) = shape_intersections
                    .shape_intersections(
                        &Collider::rectangle(collider_size, collider_size),
                        transform.translation.xy()
                            + Vec2::new(collider_size * 2.5 * direction.get(), 0.),
                        0.,
                        CollisionGroup::filter(ENEMY),
                    )
                    .first()
                {
                    println!("Slashed: {other:?}",);
                    behavior.has_hit = true;
                };
            }
            Stage::Settle if timer_finished => {
                behavior.set_stage(Stage::Dormant);
            }
            _ => {}
        }
    }
}

pub struct SlashingBehavior;

impl Plugin for SlashingBehavior {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, demo_slash_player_behavior);
    }
}

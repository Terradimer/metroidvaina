use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use itertools::izip;
use leafwing_input_manager::action_state::ActionState;

use crate::input::Inputs;

use super::{
    components::{StuckCheck, UpperCollider},
    PLAYER_KICK_SPEED,
};

#[derive(Component)]
pub struct Grounded {
    in_state: bool,
}

impl Grounded {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self { in_state: false }
    }
}

#[derive(Component)]
pub struct Crouching {
    in_state: bool,
    pub stuck: bool,
}

pub fn crouching_state_change(
    mut q_state: Query<&mut Crouching>,
    mut q_collider: Query<&mut CollisionGroups, With<UpperCollider>>,
    q_stuck: Query<Entity, With<StuckCheck>>,
    rapier_context: Res<RapierContext>,
) {
    for (mut state, mut group, stuck_check) in izip!(
        &mut q_state.iter_mut(),
        &mut q_collider.iter_mut(),
        q_stuck.iter()
    ) {
        if state.is_changed() || state.stuck {
            if state.check() {
                group.memberships = Group::GROUP_3;
            } else {
                if rapier_context
                    .intersection_pairs_with(stuck_check)
                    .any(|(_, _, intersecting)| intersecting)
                {
                    state.in_state = true;
                    state.stuck = true;
                    group.memberships = Group::GROUP_3;
                } else {
                    group.memberships = Group::GROUP_2;
                    state.stuck = false;
                }
            }
        }
    }
}

impl Crouching {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn new() -> Self {
        Self {
            in_state: false,
            stuck: false,
        }
    }
}

#[derive(Component)]
pub struct Kicking {
    in_state: bool,
}

pub fn kicking_state_change(
    input: Res<ActionState<Inputs>>,
    mut q_state: Query<(&mut Velocity, &mut GravityScale, Ref<Kicking>)>,
) {
    let move_axis = match input.clamped_axis_pair(&Inputs::Directional) {
        Some(data) => data.xy(),
        None => return,
    };

    for (mut velocity, mut gravity, state) in q_state.iter_mut() {
        let vel = &mut velocity.linvel;

        if state.is_changed() {
            if state.check() {
                gravity.0 = 0.;
                if vel.x.signum() * move_axis.x.signum() < -0.2 || vel.x.abs() < PLAYER_KICK_SPEED {
                    vel.x =
                        PLAYER_KICK_SPEED * move_axis.x.abs().ceil().copysign(move_axis.x) * 1.1;
                }
                vel.y = -PLAYER_KICK_SPEED;
            } else {
                gravity.0 = 1.;
            }
        }
    }
}

impl Kicking {
    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        }
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        }
    }

    pub fn check(&self) -> bool {
        self.in_state
    }
    pub fn new() -> Self {
        Self { in_state: false }
    }
}

#[derive(Component)]
pub struct Jumping {
    pub air_jumps_remaining: i32,
    pub air_jumps_max: i32,
    in_state: bool,
}

pub fn jumping_state_change(mut q_state: Query<(&mut Velocity, &Grounded, &mut Jumping)>) {
    for (mut velocity, grounded, mut state) in q_state.iter_mut() {
        let vel = &mut velocity.linvel;
        if state.is_changed() {
            if state.check() {
                vel.y = 600.;
                if !grounded.check() {
                    state.air_jumps_remaining -= 1;
                }
            } else {
                vel.y /= 2.;
            }
        }
    }
}

impl Jumping {
    pub fn can_jump(&self) -> bool {
        self.air_jumps_remaining > 0 && !self.in_state
    }

    pub fn refill_jumps(&mut self) {
        self.air_jumps_remaining = self.air_jumps_max;
    }

    pub fn has_air_jumped(&self) -> bool {
        self.air_jumps_remaining < self.air_jumps_max
    }

    pub fn check(&self) -> bool {
        self.in_state
    }

    pub fn start(&mut self) {
        if !self.in_state {
            self.in_state = true
        };
    }

    pub fn stop(&mut self) {
        if self.in_state {
            self.in_state = false
        };
    }

    pub fn new(max_air_jumps: i32) -> Self {
        Jumping {
            air_jumps_remaining: 0,
            air_jumps_max: max_air_jumps,
            in_state: false,
        }
    }
}

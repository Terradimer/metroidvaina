use bevy_rapier2d::geometry::{CollisionGroups, Group};

pub struct Groups;

impl Groups {
    const NONE: Group = Group::GROUP_1; // Do not use this for anything other than for empty filters
    const INACTIVE: Group = Group::GROUP_2;
    pub const PLAYER: Group = Group::GROUP_3;
    pub const ENEMY: Group = Group::GROUP_4;
    pub const ENVIRONMENT: Group = Group::GROUP_5;
    const COLLISION: Group = Group::GROUP_6;
    const HIT: Group = Group::GROUP_7;

    // Helper methods to create collision groups for different entities

    pub fn inactive() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::INACTIVE,
            filters: Self::NONE,
        }
    }

    pub fn hitbox(target: Group) -> CollisionGroups {
        CollisionGroups {
            memberships: Self::HIT,
            filters: target,
        }
    }

    pub fn hurtbox(target: Group) -> CollisionGroups {
        CollisionGroups {
            memberships: target,
            filters: Self::HIT,
        }
    }

    pub fn collision() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::COLLISION,
            filters: Self::ENVIRONMENT,
        }
    }

    pub fn environment() -> CollisionGroups {
        CollisionGroups {
            memberships: Self::ENVIRONMENT,
            filters: Self::COLLISION | Self::HIT,
        }
    }
}

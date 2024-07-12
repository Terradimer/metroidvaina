use avian2d::{
    collision::{CollisionLayers, LayerMask},
    prelude::PhysicsLayer,
};

pub struct CollisionGroups;

#[derive(PhysicsLayer, Clone, Copy)]
pub enum Group {
    Player,
    Enemy,
    Environment,
}

impl Group {
    fn to_mask(self) -> u32 {
        match self {
            Group::Player => CollisionGroups::PLAYER,
            Group::Enemy => CollisionGroups::ENEMY,
            Group::Environment => CollisionGroups::ENVIRONMENT,
        }
    }
}

impl CollisionGroups {
    const NONE: u32 = 0;
    const INACTIVE: u32 = 1 << 0;
    const COLLISION: u32 = 1 << 1;
    const PLAYER: u32 = 1 << 2;
    const ENEMY: u32 = 1 << 3;
    const ENVIRONMENT: u32 = 1 << 4;
    const HIT: u32 = 1 << 5;

    const INACTIVE_INTERACTION: CollisionLayers = CollisionLayers {
        memberships: LayerMask { 0: Self::INACTIVE },
        filters: LayerMask { 0: Self::NONE },
    };

    const COLLISION_INTERACTION: CollisionLayers = CollisionLayers {
        memberships: LayerMask { 0: Self::COLLISION },
        filters: LayerMask {
            0: Self::ENVIRONMENT,
        },
    };

    const ENVIRONMENT_INTERACTION: CollisionLayers = CollisionLayers {
        memberships: LayerMask {
            0: Self::ENVIRONMENT,
        },
        filters: LayerMask { 0: Self::COLLISION },
    };

    pub fn hitbox(targets: &[Group]) -> CollisionLayers {
        let target_mask = targets.iter().fold(0, |acc, &group| acc | group.to_mask());
        CollisionLayers::new(Self::HIT, target_mask)
    }

    pub fn hurtbox(sources: &[Group]) -> CollisionLayers {
        let source_mask = sources.iter().fold(0, |acc, &group| acc | group.to_mask());
        CollisionLayers::new(source_mask, Self::HIT)
    }

    pub const fn inactive() -> CollisionLayers {
        Self::INACTIVE_INTERACTION
    }

    pub const fn collision() -> CollisionLayers {
        Self::COLLISION_INTERACTION
    }

    pub const fn environment() -> CollisionLayers {
        Self::ENVIRONMENT_INTERACTION
    }
}

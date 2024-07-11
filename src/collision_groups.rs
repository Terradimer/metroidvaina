use avian2d::collision::CollisionLayers;
use avian2d::prelude::PhysicsLayer;

#[derive(PhysicsLayer)]
pub enum Group {
    NONE,
    INACTIVE,
    PLAYER,
    ENEMY,
    ENVIRONMENT,
    COLLISION,
    HIT,
}

pub struct Groups;

impl Groups {
    const NONE: Group = Group::NONE; // Do not use this for anything other than for empty filters
    const INACTIVE: Group = Group::INACTIVE;
    pub const PLAYER: Group = Group::PLAYER;
    pub const ENEMY: Group = Group::ENEMY;
    pub const ENVIRONMENT: Group = Group::ENVIRONMENT;
    const COLLISION: Group = Group::COLLISION;
    const HIT: Group = Group::HIT;

    // Helper methods to create collision groups for different entities

    pub fn inactive() -> CollisionLayers {
        CollisionLayers::new(Self::INACTIVE, Self::NONE)
    }

    pub fn hitbox(target: Group) -> CollisionLayers {
        CollisionLayers::new(Self::HIT, target)
    }

    pub fn hurtbox(target: Group) -> CollisionLayers {
        CollisionLayers::new(target, Self::HIT)
    }

    pub fn collision() -> CollisionLayers {
        CollisionLayers::new(Self::COLLISION, Self::ENVIRONMENT)
    }

    pub fn environment() -> CollisionLayers {
        CollisionLayers::new(Self::ENVIRONMENT, [Self::COLLISION, Self::HIT])
    }
}

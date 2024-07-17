use avian2d::{
    collision::{CollisionLayers, LayerMask},
    spatial_query::SpatialQueryFilter,
};

const NONE: LayerMask = LayerMask(0);
const INACTIVE: LayerMask = LayerMask(1);
pub const PLAYER: LayerMask = LayerMask(1 << 1);
pub const ENEMY: LayerMask = LayerMask(1 << 2);
pub const ENVIRONMENT: LayerMask = LayerMask(1 << 3);
const COLLIDER: LayerMask = LayerMask(1 << 4);

pub struct CollisionGroup;

impl CollisionGroup {
    pub const INACTIVE: CollisionLayers = CollisionLayers {
        memberships: INACTIVE,
        filters: NONE,
    };

    pub const ENVIRONMENT: CollisionLayers = CollisionLayers {
        memberships: ENVIRONMENT,
        filters: COLLIDER,
    };

    pub const COLLIDER: CollisionLayers = CollisionLayers {
        memberships: COLLIDER,
        filters: ENVIRONMENT,
    };

    pub fn hurtbox(groups: LayerMask) -> CollisionLayers {
        CollisionLayers {
            memberships: groups,
            filters: NONE,
        }
    }

    pub fn filter(groups: LayerMask) -> SpatialQueryFilter {
        SpatialQueryFilter::from_mask(groups)
    }
}

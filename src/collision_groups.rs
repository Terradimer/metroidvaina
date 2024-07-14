
use avian2d::{
    collision::{CollisionLayers, LayerMask}, spatial_query::SpatialQueryFilter}
;

#[allow(dead_code)]
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum Group {
    None        = 0b0,
    Hitbox      = 0b1,
    Hurtbox     = 0b10,
    Environment = 0b100,
    Collider    = 0b1000,
}

impl Into<SpatialQueryFilter> for Group {
    fn into(self) -> SpatialQueryFilter {
        SpatialQueryFilter::from_mask(self)
    }
}

impl Into<LayerMask> for Group {
    fn into(self) -> LayerMask {
        self.to_layer_mask()
    }
}

impl Group {
    pub const fn to_layer_mask(self)  -> LayerMask {
        LayerMask(self as u32)
    }

    pub const fn inactive() -> CollisionLayers {
        CollisionLayers {
            memberships: Self::None.to_layer_mask(),
            filters: Self::None.to_layer_mask(),
        }
    }

    pub const fn environment() -> CollisionLayers {
        CollisionLayers {
            memberships: Self::Environment.to_layer_mask(),
            filters: Self::Collider.to_layer_mask(),
        }
    }

    pub const fn collider() -> CollisionLayers {
        CollisionLayers {
            memberships: Self::Collider.to_layer_mask(),
            filters: Self::Environment.to_layer_mask(),
        }
    }

    pub const fn hurtbox() -> CollisionLayers {
        CollisionLayers {
            memberships: Self::Hurtbox.to_layer_mask(),
            filters: Self::None.to_layer_mask(),
        }
    }
}
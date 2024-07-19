use avian2d::parry::shape::TypedShape;
use bevy::color::palettes::css;
use bevy::prelude::*;
use avian2d::prelude::*;
use bevy_ecs::system::SystemParam;

#[derive(SystemParam)]
pub struct ShapeIntersections<'w, 's> {
    pub spatial_query: SpatialQuery<'w, 's>,
    pub gizmos: Gizmos<'w, 's>,
}

impl ShapeIntersections<'_, '_> {
    pub fn shape_intersections(
        &mut self,
        shape: &Collider,
        shape_position: avian2d::math::Vector,
        shape_rotation: avian2d::math::Scalar,
        query_filter: SpatialQueryFilter,
    ) -> Vec<Entity> {
        match shape.shape_scaled().as_typed_shape() {
            TypedShape::Cuboid(s) => 
            self.gizmos.primitive_2d(&Rectangle { half_size: s.half_extents.into()}, shape_position, shape_rotation, css::YELLOW),
            x => panic!("Debug rendering for {:?} is not yet implemented. Consider implementing it in {}, using a different implemented collider shape, or replacing ShapeIntersections with SpatialQuery to disable debug rendering.", x, file!())
        }
        self.spatial_query.shape_intersections(shape, shape_position, shape_rotation, query_filter)
    }
}
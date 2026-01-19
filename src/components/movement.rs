use bevy::{ecs::component::Component, math::Vec3};

/// Marking a TileCoordinates component with Movement
/// will cause it to move to tile coordinates offset
/// with the specified offset. When the animation is
/// finished, the TileCoordinates should be updated
/// to reflect the final position after the movment.
#[derive(Component, Clone, Debug)]
pub struct Movement {
    pub offset: Vec3,
    pub movement_speed: f32,
    pub animation_percentage: f32,
}

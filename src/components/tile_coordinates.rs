use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug)]
pub(crate) struct TileCoordinates {
    pub(crate) y: isize,
    pub(crate) x: isize,
    pub(crate) z: isize,
    pub(crate) visual_offset: Vec3,
    pub(crate) is_on_top: bool,
    pub(crate) movement_direction: Option<MovementDirection>,
    pub(crate) movement_animation_percentage: Option<f32>,
    pub(crate) movement_speed: f32,
    pub(crate) falling_speed: f32,
}

impl Default for TileCoordinates {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            z: 0,
            visual_offset: Vec3::ZERO,
            is_on_top: true,
            movement_direction: None,
            movement_animation_percentage: None,
            movement_speed: 1.0,
            falling_speed: 20.0,
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum MovementDirection {
    NorthWest,
    NorthEast,
    East,
    SouthEast,
    SouthWest,
    West,
    #[allow(unused)]
    Up,
    Down,
}

impl MovementDirection {
    pub(crate) fn get_tile_coordinate_offset(&self) -> (isize, isize, isize) {
        match self {
            MovementDirection::East => (1, 0, 0),
            MovementDirection::West => (-1, 0, 0),
            MovementDirection::NorthEast => (0, 0, 1),
            MovementDirection::NorthWest => (-1, 0, 1),
            MovementDirection::SouthEast => (1, 0, -1),
            MovementDirection::SouthWest => (0, 0, -1),
            MovementDirection::Up => (0, 1, 0),
            MovementDirection::Down => (0, -1, 0),
        }
    }
}

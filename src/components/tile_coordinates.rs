use bevy::{ecs::component::Component, math::Vec3};

#[derive(Component, Debug)]
pub struct TileCoordinates {
    pub y: isize,
    pub x: isize,
    pub z: isize,
    pub visual_offset: Vec3,
    pub is_on_top: bool,
    pub movement_direction: Option<MovementDirection>,
    pub movement_animation_percentage: Option<f32>,
    pub movement_speed: f32,
    pub falling_speed: f32,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum MovementDirection {
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
    pub fn get_tile_coordinate_offset(&self) -> Vec3 {
        match self {
            MovementDirection::East => Vec3::new(1., 0., 0.),
            MovementDirection::West => Vec3::new(-1., 0., 0.),
            MovementDirection::NorthEast => Vec3::new(0., 0., 1.),
            MovementDirection::NorthWest => Vec3::new(-1., 0., 1.),
            MovementDirection::SouthEast => Vec3::new(1., 0., -1.),
            MovementDirection::SouthWest => Vec3::new(0., 0., -1.),
            MovementDirection::Up => Vec3::new(0., 1., 0.),
            MovementDirection::Down => Vec3::new(0., -1., 0.),
        }
    }

    pub fn rotate_y(&self, num_rotations: isize) -> Self {
        let sorted_directions = [
            Self::East,
            Self::SouthEast,
            Self::SouthWest,
            Self::West,
            Self::NorthWest,
            Self::NorthEast,
        ];

        match sorted_directions.iter().position(|item| self == item) {
            None => self,
            Some(value) => {
                let new_index = (value as isize + num_rotations).rem_euclid(6);
                &sorted_directions[new_index as usize]
            }
        }
        .clone()
    }
}

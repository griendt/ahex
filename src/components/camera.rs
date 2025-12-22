use bevy::{ecs::component::Component, math::Quat};

#[derive(Component, Default)]
pub(crate) struct CameraAngle {
    pub(crate) y: isize,
    pub(crate) x: isize,
    pub(crate) z: isize,
    pub(crate) total_6th_rotations: isize,
    pub(crate) rotation_direction: Option<RotationDirection>,
    pub(crate) rotation_animation_percentage: Option<f32>,
    pub(crate) rotation_speed: f32,
}

#[derive(Clone)]
pub(crate) enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

impl RotationDirection {
    pub(crate) fn get_rotation_quaternion(&self) -> Quat {
        match self {
            RotationDirection::Clockwise => Quat::from_rotation_y(1.0),
            RotationDirection::CounterClockwise => Quat::from_rotation_y(-1.0),
        }
    }
}

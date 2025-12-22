use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub(crate) struct CameraAngle {
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

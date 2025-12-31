use bevy::ecs::component::Component;

#[derive(Component, Default)]
pub struct CameraAngle {
    pub total_6th_rotations: isize,
    pub rotation_direction: Option<RotationDirection>,
    pub rotation_animation_percentage: Option<f32>,
    pub rotation_speed: f32,
}

#[derive(Clone)]
pub enum RotationDirection {
    Clockwise,
    CounterClockwise,
}

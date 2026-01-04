use bevy::ecs::component::Component;

#[derive(Component)]
pub struct Sun {
    /// The time of day, should be between 0 and 1, 0.5 being mid-day.
    pub time_of_day: f32,
    pub seconds_per_day: usize,
}

use bevy::ecs::resource::Resource;
use serde::Deserialize;

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct Settings {
    pub initial_level_number: isize,
    pub display: DisplaySettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub water: WaterDisplaySettings,
    pub sun: SunDisplaySettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct WaterDisplaySettings {
    pub shadows_enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct SunDisplaySettings {
    pub height: f32,
    pub height_variation: f32,
    pub illuminance: f32,
    pub illuminance_variation: f32,
    pub initial_time_of_day: f32,
    pub seconds_per_day: usize,
    pub shadows_enabled: bool,
}

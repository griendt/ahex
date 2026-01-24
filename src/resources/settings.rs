use bevy::ecs::resource::Resource;
use serde::Deserialize;

#[derive(Resource, Deserialize, Clone, Debug)]
pub struct Settings {
    pub initial_level_number: isize,
    pub display: DisplaySettings,
    pub camera: CameraSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct DisplaySettings {
    pub width: u32,
    pub height: u32,
    pub water: WaterDisplaySettings,
    pub sun: SunDisplaySettings,
    pub level_complete: LevelCompleteDisplaySettings,
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
    pub initial_time_of_day: f32,
    pub seconds_per_day: usize,
    pub shadows_enabled: bool,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LevelCompleteDisplaySettings {
    pub hue_change_speed: f32,
    pub font_size: f32,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CameraSettings {
    pub rotation_speed: f32,
    pub bloom: CameraBloomSettings,
}

#[derive(Deserialize, Clone, Debug)]
pub struct CameraBloomSettings {
    pub intensity: f32,
    pub low_frequency_boost: f32,
    pub low_frequency_boost_curvature: f32,
    pub high_pass_frequency: f32,
}

use bevy::{prelude::*, window::WindowResolution};
use bevy_water::{WaterPlugin, WaterSettings};

use crate::systems::{
    camera::move_camera,
    player::{add_player_bloom, player_controls},
    setup::setup,
    tiles::{colorize_tiles, set_tile_transform},
};

mod components;
mod systems;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::hsl(200.0, 0.1, 0.15)))
        .insert_resource(WaterSettings {
            height: 0.3,
            amplitude: 1.5,
            alpha_mode: AlphaMode::Add,
            water_quality: bevy_water::WaterQuality::High,
            deep_color: Color::hsla(180.0, 1.0, 0.3, 1.0),
            ..default()
        })
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ahex".into(),
                resizable: false,
                resolution: WindowResolution::new(1280, 720),
                canvas: Some("#bevy".to_owned()),
                desired_maximum_frame_latency: core::num::NonZero::new(1u32),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(WaterPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, colorize_tiles, add_player_bloom))
        .add_systems(Update, (set_tile_transform, player_controls).chain())
        .run();
}

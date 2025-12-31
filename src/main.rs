use bevy::{prelude::*, window::WindowResolution};
use bevy_hanabi::HanabiPlugin;
use bevy_water::{WaterPlugin, WaterSettings};

use crate::systems::{
    camera::move_camera,
    goal::{add_goal_bloom, rotate_goal, vary_goal_intensity},
    level::{build_level, restart_level},
    player::{add_player_bloom, collect_goals, player_controls},
    setup::{setup, setup_effects},
    tiles::{colorize_tiles, set_tile_transform},
};

use crate::resources::effects::GlobalEffects;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .insert_resource(GlobalEffects::default())
        .insert_resource(ClearColor(Color::hsl(200.0, 0.1, 0.15)))
        .insert_resource(WaterSettings {
            height: 0.3,
            amplitude: 1.5,
            alpha_mode: AlphaMode::Add,
            water_quality: bevy_water::WaterQuality::Basic,
            deep_color: Color::hsla(180.0, 1.0, 0.6, 1.0),
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
        .add_plugins(HanabiPlugin)
        .add_systems(Startup, (setup, setup_effects, build_level.after(setup)))
        .add_systems(
            Update,
            (
                add_player_bloom,
                add_goal_bloom,
                vary_goal_intensity,
                move_camera,
                rotate_goal,
                colorize_tiles,
                collect_goals,
                set_tile_transform,
                restart_level,
                player_controls.after(set_tile_transform),
            ),
        )
        .run();
}

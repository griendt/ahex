use bevy::{prelude::*, window::WindowResolution};
use bevy_hanabi::HanabiPlugin;
use bevy_water::{WaterPlugin, WaterSettings};

use crate::{
    resources::levels::LevelResource,
    systems::{
        camera::move_camera,
        goal::{add_goal_bloom, rotate_goal, vary_goal_intensity},
        level::{build_level, go_to_next_level, restart_level, show_level_complete},
        lighting::{create_the_sun, update_the_sun},
        player::{add_player_bloom, collect_goals, player_controls},
        setup::{enable_water_shadows, setup, setup_effects},
        tiles::{
            apply_movement_map, apply_player_movement, colorize_tiles, on_player_finished_moving,
            on_player_started_moving, set_transform_based_on_tile_coordinates,
        },
    },
};

use crate::resources::effects::GlobalEffects;

mod components;
mod resources;
mod systems;

fn main() {
    App::new()
        .insert_resource(GlobalEffects::default())
        .insert_resource(ClearColor(Color::hsl(200.0, 0.1, 0.15)))
        .insert_resource(LevelResource {
            current_level_number: 1,
            ..default()
        })
        .insert_resource(WaterSettings {
            height: 0.3,
            amplitude: 1.5,
            alpha_mode: AlphaMode::Add,
            water_quality: bevy_water::WaterQuality::Basic, // High or better for shadows
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
        .add_systems(
            Startup,
            (
                setup,
                setup_effects,
                (build_level, create_the_sun).after(setup),
            ),
        )
        .add_systems(PostStartup, enable_water_shadows)
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
                apply_player_movement,
                restart_level,
                go_to_next_level,
                show_level_complete,
                update_the_sun,
                player_controls.after(apply_player_movement),
                set_transform_based_on_tile_coordinates,
                apply_movement_map,
            ),
        )
        .add_observer(on_player_finished_moving)
        .add_observer(on_player_started_moving)
        .run();
}

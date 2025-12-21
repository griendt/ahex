use std::f32::consts::TAU;

use bevy::{
    asset::AssetServer,
    camera::Camera3d,
    color::Color,
    core_pipeline::tonemapping::Tonemapping,
    ecs::{
        bundle::Bundle,
        system::{Commands, Res},
    },
    light::DirectionalLight,
    math::{Quat, Vec2, Vec3},
    post_process::bloom::{Bloom, BloomCompositeMode, BloomPrefilter},
    scene::SceneRoot,
    transform::components::Transform,
    utils::default,
};
use bevy_gltf::GltfAssetLabel;

use crate::components::{player::Player, tile::Tile, tile_coordinates::TileCoordinates};

pub(crate) fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Spawn hexagons
    for xyzt in [
        (0, 1, 0, true),
        (0, 0, 0, true),
        (1, 0, 0, true),
        (2, 0, 0, true),
        (3, 0, 0, false),
        (3, 0, 1, false),
        (3, 0, 2, true),
        (4, 0, 3, true),
        (4, 0, 0, true),
        (4, -1, -1, true),
        (4, -2, -2, true),
        (2, 1, 0, false),
        (2, 1, 1, true),
        (2, 2, 0, false),
        (2, 2, 1, false),
        (2, 2, 2, true),
        (1, 2, 0, false),
        (1, 2, 1, false),
        (1, 2, 2, false),
        (1, 2, 3, true),
        (0, 2, 0, false),
        (0, 2, 1, false),
        (0, 2, 2, false),
        (0, 2, 3, false),
        (0, 2, 4, true),
    ] {
        commands.spawn(create_hexagon(xyzt, &asset_server));
    }

    // Spawn player on top of somewhere
    commands.spawn((
        Player {},
        SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("ball.glb"))),
        TileCoordinates {
            x: 1,
            y: 0,
            z: 0,
            movement_speed: 3.0,
            ..default()
        },
        Transform {
            rotation: Quat::from_rotation_x(TAU / 4.0),
            scale: Vec3::new(0.5, 0.5, 0.5),
            ..default()
        },
    ));

    // Spawn a camera looking at the entities to show what's happening in this example.
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, -10.0, 20.0).looking_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y),
        Tonemapping::TonyMcMapface,
        Bloom {
            intensity: 0.3,
            low_frequency_boost: 0.9,
            low_frequency_boost_curvature: 0.85,
            high_pass_frequency: 0.5,
            prefilter: BloomPrefilter {
                threshold: 0.0,
                threshold_softness: 0.0,
            },
            composite_mode: BloomCompositeMode::EnergyConserving,
            max_mip_dimension: 512,
            scale: Vec2::ONE,
        },
    ));

    // Add a light source so we can see clearly.
    commands.spawn((
        DirectionalLight {
            shadows_enabled: true,
            soft_shadow_size: None,
            affects_lightmapped_mesh_diffuse: false,
            illuminance: 5000.0,
            ..default()
        },
        Transform::from_xyz(-60.0, -10.0, 50.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

fn create_hexagon(
    xyzt: (isize, isize, isize, bool),
    asset_server: &Res<AssetServer>,
) -> impl Bundle {
    let tile_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile.glb"));
    let tile_below_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile_below.glb"));

    (
        Tile {
            color: Color::hsla(
                (xyzt.0 * 60 + xyzt.1 * 30) as f32,
                0.8,
                0.6 + 0.1 * xyzt.2 as f32,
                1.0,
            ),
        },
        TileCoordinates {
            x: xyzt.0,
            y: xyzt.1,
            z: xyzt.2,
            is_on_top: xyzt.3,
            ..default()
        },
        SceneRoot(if xyzt.3 {
            tile_asset.clone()
        } else {
            tile_below_asset.clone()
        }),
        Transform {
            rotation: Quat::from_rotation_x(TAU / 4.0),
            scale: Vec3::new(1.0, 1.0, 1.0),
            ..default()
        },
    )
}

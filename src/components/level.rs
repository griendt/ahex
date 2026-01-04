use bevy::{
    asset::AssetServer,
    color::{Color, palettes::css::YELLOW},
    ecs::{
        component::Component,
        system::{Commands, Res},
    },
    light::PointLight,
    log::info,
    math::Vec3,
    scene::SceneRoot,
    text::TextFont,
    transform::components::Transform,
    ui::{
        Node, PositionType, px,
        widget::{Text, TextShadow},
    },
    utils::default,
};
use bevy_gltf::GltfAssetLabel;
use serde::Deserialize;

use crate::components::{
    goal::Goal,
    player::{Player, PlayerFinishedMoving},
    tile::{MovementMap, Tile},
    tile_coordinates::TileCoordinates,
};

#[derive(Component)]
pub struct LevelEntityMarker;

#[derive(Component)]
pub struct HelpTextMarker;

#[derive(Component)]
pub struct LevelCompleteTextMarker;

#[derive(Component, Deserialize, Debug)]
pub struct LevelParser {
    pub metadata: LevelMetadata,
    pub layers: Vec<LevelLayer>,
}

#[derive(Deserialize, Debug)]
pub struct LevelMetadata {
    pub name: String,
    pub biome: LevelMetadataBiome,
    pub help_text: String,
}

#[derive(Deserialize, Debug)]
pub enum LevelMetadataBiome {
    DAYLIGHT,
    DUSK,
    NIGHT,
}

#[derive(Deserialize, Debug)]
pub struct LevelLayer {
    pub pillars: Option<bool>,
    pub height_map: String,
    pub modifiers: Vec<String>,
    pub movement_maps: Option<Vec<Vec<(isize, isize, isize)>>>,
}

impl LevelParser {
    pub fn render_level(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
        if !self.metadata.help_text.is_empty() {
            commands.spawn((
                HelpTextMarker,
                LevelEntityMarker,
                Text::new(self.metadata.help_text.as_str()),
                TextFont {
                    font: asset_server.load("fonts/main.ttf"),
                    font_size: 48.0,
                    ..default()
                },
                TextShadow::default(),
                Node {
                    position_type: PositionType::Absolute,
                    top: px(20),
                    left: px(20),
                    ..default()
                },
            ));
        }

        let (x_offset, z_offset) = self.get_level_xz_offsets();

        for layer in &self.layers {
            let heights: Vec<Vec<char>> = layer
                .height_map
                .trim()
                .split('\n')
                .map(|line| line.chars().collect())
                .collect();

            let modifier_maps: Vec<Vec<Vec<char>>> = layer
                .modifiers
                .iter()
                .map(|modifier_map| {
                    modifier_map
                        .trim()
                        .split('\n')
                        .map(|line| line.chars().collect())
                        .collect()
                })
                .collect();

            let mut num_movement_maps_applied = 0;

            for (row_index, row) in heights.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    let tile_xyz = (
                        col_index as isize - x_offset,
                        char.to_digit(10).unwrap_or(0) as isize,
                        z_offset - row_index as isize,
                    );

                    let mut movement_map: Vec<(isize, isize, isize)> = vec![];

                    for modifier_map in &modifier_maps {
                        let modifier = modifier_map
                            .iter()
                            .nth(row_index)
                            .expect("Modifier map has fewer rows than height map")
                            .iter()
                            .nth(col_index)
                            .expect("Modifier map has fewer columns than height map");

                        match modifier {
                            'P' => {
                                self.get_player_entity(
                                    tile_xyz.0,
                                    tile_xyz.1,
                                    tile_xyz.2,
                                    commands,
                                    asset_server,
                                );
                            }
                            'G' => {
                                self.get_goal_entity(
                                    tile_xyz.0,
                                    tile_xyz.1,
                                    tile_xyz.2,
                                    commands,
                                    asset_server,
                                );
                            }
                            'M' => {
                                movement_map =
                                    layer.movement_maps
                                        .clone()
                                        .expect("Movement map modifier found, but no movement maps specified")
                                        .iter()
                                        .nth(num_movement_maps_applied)
                                        .expect("Too few movement maps specified")
                                        .clone();
                            }
                            _ => continue,
                        };
                    }

                    if *char != '.' {
                        self.get_tile_entity(
                            tile_xyz.0,
                            tile_xyz.1,
                            tile_xyz.2,
                            true,
                            layer.pillars.unwrap_or(false),
                            movement_map,
                            commands,
                            asset_server,
                        );
                    }
                }
            }
        }

        // Fire this event so that stuff that normally occurs after each step,
        // also occurs before the first step. This puts tiles into place, starts
        // the first movements and so on.
        commands.trigger(PlayerFinishedMoving {});
    }

    fn get_level_xz_offsets(&self) -> (isize, isize) {
        let level_width = self
            .layers
            .iter()
            .map(|layer| {
                layer
                    .height_map
                    .trim()
                    .split('\n')
                    .map(|line| line.len())
                    .max()
                    .unwrap()
            })
            .max()
            .unwrap();

        let level_height = self
            .layers
            .iter()
            .map(|layer| {
                layer
                    .height_map
                    .trim()
                    .split('\n')
                    .collect::<Vec<_>>()
                    .len()
            })
            .max()
            .unwrap();

        info!("Level width is {level_width} and level height is {level_height}");

        (level_width as isize / 2, level_height as isize / 2)
    }

    fn get_player_entity(
        &self,
        x: isize,
        y: isize,
        z: isize,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        commands.spawn((
            Player {},
            LevelEntityMarker,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("ball.glb"))),
            TileCoordinates {
                x: x,
                y: y,
                z: z,
                movement_speed: 5.0,
                ..default()
            },
            Transform {
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..default()
            },
        ));
    }

    fn get_goal_entity(
        &self,
        x: isize,
        y: isize,
        z: isize,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        commands.spawn((
            Goal {},
            LevelEntityMarker,
            SceneRoot(asset_server.load(GltfAssetLabel::Scene(0).from_asset("goal.glb"))),
            TileCoordinates {
                x: x,
                y: y,
                z: z,
                visual_offset: Vec3::new(0.0, 0.5, 0.0),
                ..default()
            },
            Transform {
                scale: Vec3::new(0.5, 0.5, 0.5),
                ..default()
            },
            PointLight {
                intensity: 500_000.0,
                color: YELLOW.into(),
                shadows_enabled: true,
                ..default()
            },
        ));
    }

    fn get_tile_entity(
        &self,
        x: isize,
        y: isize,
        z: isize,
        is_on_top: bool,
        is_pillar: bool,
        movement_map: Vec<(isize, isize, isize)>,
        commands: &mut Commands,
        asset_server: &Res<AssetServer>,
    ) {
        let tile_asset = asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile.glb"));
        let tile_below_asset =
            asset_server.load(GltfAssetLabel::Scene(0).from_asset("tile_below.glb"));

        let bundle = (
            Tile {
                color: Color::hsla(90.0, 0.8, (0.4 + 0.1 * y as f32).clamp(0.05, 1.0), 1.0),
            },
            LevelEntityMarker,
            TileCoordinates {
                x: x,
                y: y,
                z: z,
                is_on_top: is_on_top,
                ..default()
            },
            SceneRoot(if is_on_top {
                tile_asset.clone()
            } else {
                tile_below_asset.clone()
            }),
            Transform::default(),
            MovementMap {
                map: movement_map.clone(),
                index: 0,
            },
        );

        commands.spawn(bundle);

        if y > 0 && is_pillar {
            self.get_tile_entity(
                x,
                y - 1,
                z,
                false,
                is_pillar,
                movement_map,
                commands,
                asset_server,
            );
        }
    }
}

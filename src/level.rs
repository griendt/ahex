use bevy::{
    asset::AssetServer,
    color::{Color, palettes::css::YELLOW},
    ecs::system::{Commands, Res},
    light::PointLight,
    log::info,
    math::Vec3,
    scene::SceneRoot,
    transform::components::Transform,
    utils::default,
};
use bevy_gltf::GltfAssetLabel;
use serde::Deserialize;

use crate::components::{
    goal::Goal, player::Player, tile::Tile, tile_coordinates::TileCoordinates,
};

#[derive(Deserialize, Debug)]
pub struct LevelParser {
    pub metadata: LevelMetadata,
    pub layers: Vec<LevelLayer>,
}

#[derive(Deserialize, Debug)]
pub struct LevelMetadata {
    pub name: String,
    pub biome: LevelMetadataBiome,
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
}

impl LevelParser {
    pub fn render_level(&self, commands: &mut Commands, asset_server: &Res<AssetServer>) {
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

            for (row_index, row) in heights.iter().enumerate() {
                for (col_index, char) in row.iter().enumerate() {
                    let tile_xyz = (
                        col_index as isize - x_offset,
                        char.to_digit(10).unwrap_or(0) as isize,
                        z_offset - row_index as isize,
                    );

                    if *char != '.' {
                        self.get_tile_entity(
                            tile_xyz.0,
                            tile_xyz.1,
                            tile_xyz.2,
                            true,
                            layer.pillars.unwrap_or(false),
                            commands,
                            asset_server,
                        );
                    }

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
                            _ => continue,
                        };
                    }
                }
            }
        }
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
        );

        commands.spawn(bundle);

        if y > 0 && is_pillar {
            self.get_tile_entity(x, y - 1, z, false, is_pillar, commands, asset_server);
        }
    }
}

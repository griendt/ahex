use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};

use crate::{
    components::{
        movement::Movement,
        player::{Player, PlayerFinishedMoving, PlayerStartedMoving},
        tile::{Carriable, MovementMap, ShouldRenderMovementMapPolylines, Tile},
        tile_coordinates::{TileCoordinates, tile_coordinates_to_transform_coordinates},
    },
    resources::levels::LevelState,
};

use crate::resources::levels::LevelResource;

pub fn colorize_tiles(
    mut commands: Commands,
    query: Query<(Entity, &Tile)>,
    children: Query<&Children>,
    mesh_materials: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, tile) in &query {
        for descendant in children.iter_descendants(entity) {
            let Ok((id, material_name)) = mesh_materials.get(descendant) else {
                continue;
            };

            let Some(material) = asset_materials.get_mut(id.id()) else {
                continue;
            };

            if material_name.0.as_str() == "Top" {
                if material.base_color != tile.color {
                    // TODO: do not create a new material each time
                    let mut new_material = material.clone();
                    new_material.base_color = tile.color;
                    // new_material.emissive = LinearRgba::rgb(0.01, 0.01, 0.01);

                    commands
                        .entity(descendant)
                        .insert(MeshMaterial3d(asset_materials.add(new_material)));
                }
            }
        }
    }
}

pub fn draw_moving_tiles_polylines(
    mut commands: Commands,
    mut polyline_materials: ResMut<Assets<PolylineMaterial>>,
    mut polylines: ResMut<Assets<Polyline>>,
    tiles: Query<(Entity, &TileCoordinates, &MovementMap), With<ShouldRenderMovementMapPolylines>>,
) {
    let polyline_material_handle = PolylineMaterialHandle(
        polyline_materials.add(PolylineMaterial {
            width: 300.0,
            color: Hsla {
                hue: 0.0,
                saturation: 1.0,
                lightness: 1.0,
                alpha: 0.1,
            }
            .into(),
            perspective: true,
            ..default()
        }),
    );

    for (entity, tile_coordinates, movement_map) in tiles {
        let mut vertex = Vec3::new(
            tile_coordinates.x as f32,
            tile_coordinates.y as f32,
            tile_coordinates.z as f32,
        );

        let mut vertices = vec![tile_coordinates_to_transform_coordinates(&vertex)];

        for offset in &movement_map.map {
            vertex += Vec3::new(offset.0 as f32, offset.1 as f32, offset.2 as f32);
            vertices.push(tile_coordinates_to_transform_coordinates(&vertex));
        }

        commands.spawn(PolylineBundle {
            polyline: PolylineHandle(polylines.add(Polyline { vertices: vertices })),
            material: polyline_material_handle.clone(),
            ..default()
        });

        commands
            .entity(entity)
            .remove::<ShouldRenderMovementMapPolylines>();
    }
}

pub fn on_player_finished_moving(
    _event: On<PlayerFinishedMoving>,
    mut level: ResMut<LevelResource>,
) {
    level.level_state = LevelState::ProcessingLevelEffects;
}

pub fn on_player_started_moving(_event: On<PlayerStartedMoving>, mut level: ResMut<LevelResource>) {
    level.level_state = LevelState::ProcessingPlayerInput;
}

pub fn apply_movement_map(
    query: Query<(&mut Transform, &mut TileCoordinates, &mut MovementMap), Without<Carriable>>,
    carriables: Query<(&TileCoordinates, &Carriable, Entity), Without<Movement>>,
    timer: Res<Time>,
    mut commands: Commands,
    mut level: ResMut<LevelResource>,
) {
    if !matches!(level.level_state, LevelState::ProcessingLevelEffects) {
        return;
    }

    let sqrt3 = 3f32.sqrt();
    let mut exists_unfinished_movement_map = false;

    for (mut transform, mut tile, mut movement_map) in query {
        if movement_map.map.is_empty() {
            continue;
        }

        let offset = movement_map.map[movement_map.index % movement_map.map.len()];
        let animation_percentage = tile
            .movement_animation_percentage
            .unwrap_or_default()
            .clamp(0.0, 1.0);

        for (carriable_coordinates, _carriable, carriable_entity) in carriables {
            if carriable_coordinates.x == tile.x
                && carriable_coordinates.y == tile.y
                && carriable_coordinates.z == tile.z
            {
                // NOTE: The player starts moving here but we do NOT fire the `PlayerStartedMoving` event.
                // This is because we want to trigger this event only if caused by the player pressing a movement key.
                commands.entity(carriable_entity).insert(Movement {
                    offset: Vec3::new(offset.0 as f32, offset.1 as f32, offset.2 as f32),
                    movement_speed: tile.movement_speed,
                    animation_percentage: animation_percentage,
                });
            }
        }

        // If animation is finished, set the actual coordinates and stop animating.
        if animation_percentage >= 1.0 {
            tile.x += offset.0;
            tile.y += offset.1;
            tile.z += offset.2;
            movement_map.index += 1;
            tile.movement_animation_percentage = None;
        } else {
            exists_unfinished_movement_map = true;
        }

        transform.translation = tile_coordinates_to_transform_coordinates(&Vec3::new(
            tile.x as f32,
            tile.y as f32,
            tile.z as f32,
        ));

        // A tile's width is 0.6 so if an object is supposed to be on top of the tile, grant it +0.2 on the z-axis.
        if tile.is_on_top {
            transform.translation.y += 0.6;
        }

        // Display the entity percentually towards the destination coordinates, if animating.
        if animation_percentage < 1.0 {
            transform.translation.x +=
                animation_percentage * (sqrt3 * (offset.0 as f32 + offset.2 as f32 / 2f32) as f32);
            transform.translation.y += animation_percentage * (0.8 * offset.1 as f32);
            transform.translation.z += animation_percentage * (offset.2 as f32 * -1.5);
        }

        // Apply any further visual offset
        transform.translation += tile.visual_offset;

        tile.movement_animation_percentage = Some(
            (tile.movement_animation_percentage.unwrap_or_default()
                + tile.movement_speed * timer.delta_secs())
            .clamp(0.0, 1.0),
        );
    }

    // If all movement maps have finished, we can process the new player input.
    if !exists_unfinished_movement_map {
        level.level_state = LevelState::WaitingForPlayerInput;
    }
}

pub fn apply_player_movement(
    mut commands: Commands,
    players: Query<(&mut TileCoordinates, Option<&mut Movement>, Entity), With<Player>>,
    tiles: Query<(&Tile, &TileCoordinates), Without<Player>>,
    level: Res<LevelResource>,
    timer: Res<Time>,
) {
    let mut all_moving_players_finished_moving = true;

    for (mut player_tile, movement, entity) in players {
        match movement {
            Some(mut movement) => {
                let animation_percentage = movement.animation_percentage;
                movement.animation_percentage = (movement.animation_percentage
                    + movement.movement_speed * timer.delta_secs())
                .clamp(0., 1.);

                // If animation is finished, set the actual coordinates and stop animating.
                if animation_percentage >= 1.0 {
                    player_tile.x += movement.offset.x as isize;
                    player_tile.y += movement.offset.y as isize;
                    player_tile.z += movement.offset.z as isize;

                    movement.animation_percentage = 0.0;
                    commands.entity(entity).remove::<Movement>();

                    // If there is no tile at the destination tile, the player is going to fall.
                    // This also means we should not yet trigger a `PlayerFinishedMoving` event.
                    // NOTE: We only do this in the `ProcessingPlayerInput` phase right now, because
                    // otherwise there is a race condition with moving tiles that finish moving and have their coordinates updated.
                    // This check should technically come _after_ those tiles are done moving.
                    if !tiles.iter().any(|tile| {
                        tile.1.is_on_top
                            && tile.1.x == player_tile.x
                            && tile.1.y == player_tile.y
                            && tile.1.z == player_tile.z
                    }) && matches!(level.level_state, LevelState::ProcessingPlayerInput)
                    {
                        commands.entity(entity).insert(Movement {
                            offset: Vec3::new(0., -1., 0.),
                            movement_speed: player_tile.falling_speed,
                            animation_percentage: 0.0,
                        });

                        all_moving_players_finished_moving = false;
                    }
                } else {
                    all_moving_players_finished_moving = false;
                }
            }
            None => {}
        }
    }

    if all_moving_players_finished_moving
        && matches!(level.level_state, LevelState::ProcessingPlayerInput)
    {
        commands.trigger(PlayerFinishedMoving {});
    }
}

pub fn set_transform_based_on_tile_coordinates(
    query: Query<(&mut Transform, &TileCoordinates, Option<&Movement>)>,
) {
    let sqrt3 = 3f32.sqrt();

    for (mut transform, tile, movement) in query {
        transform.translation = tile_coordinates_to_transform_coordinates(&Vec3::new(
            tile.x as f32,
            tile.y as f32,
            tile.z as f32,
        ));

        if tile.is_on_top {
            transform.translation.y += 0.6;
        }

        // Display the entity percentually towards the destination coordinates, if animating.
        match movement {
            Some(movement) => {
                transform.translation.x += movement.animation_percentage
                    * (sqrt3 * (movement.offset.x + movement.offset.z / 2.0));
                transform.translation.y += movement.animation_percentage * 0.8 * movement.offset.y;
                transform.translation.z += movement.animation_percentage * -1.5 * movement.offset.z;
            }
            None => {}
        }

        // Apply any further visual offset
        transform.translation += tile.visual_offset;
    }
}

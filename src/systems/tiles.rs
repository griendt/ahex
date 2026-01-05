use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;

use crate::{
    components::{
        movement::Movement,
        player::{Player, PlayerFinishedMoving, PlayerStartedMoving},
        tile::{MovementMap, Tile},
        tile_coordinates::TileCoordinates,
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
    query: Query<(&mut Transform, &mut TileCoordinates, &mut MovementMap), Without<Player>>,
    carriables: Query<(&TileCoordinates, &Player, Entity), Without<Movement>>,
    timer: Res<Time>,
    mut commands: Commands,
    mut level: ResMut<LevelResource>,
) {
    if !matches!(level.level_state, LevelState::ProcessingLevelEffects) {
        return;
    }

    let sqrt3 = 3f32.sqrt();

    for (mut transform, mut tile, mut movement_map) in query {
        let offset = match movement_map.map.is_empty() {
            false => movement_map.map[movement_map.index % movement_map.map.len()],
            true => (0, 0, 0),
        };

        let animation_percentage = tile.movement_animation_percentage.unwrap_or_default();

        for (player_coordinates, _player, player_entity) in carriables {
            if player_coordinates.x == tile.x
                && player_coordinates.y == tile.y
                && player_coordinates.z == tile.z
            {
                // NOTE: The player starts moving here but we do NOT fire the `PlayerStartedMoving` event.
                // This is because we want to trigger this event only if caused by the player pressing a movement key.
                commands.entity(player_entity).insert(Movement {
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

            // TODO: what if different entities finish at a different time?
            level.level_state = LevelState::WaitingForPlayerInput;
        }

        // An offset in the z-coordinate will move it to the right and up (visually); we use hexagonal geometry with pointy tops.
        transform.translation.x = ((tile.x as f32) + (tile.z as f32) / 2f32) * sqrt3;
        transform.translation.z = (tile.z as f32) * -1.5;

        // A tile's width is 0.6 so if an object is supposed to be on top of the tile, grant it +0.2 on the z-axis.
        transform.translation.y = 0.8 * tile.y as f32 + if tile.is_on_top { 0.6 } else { 0.0 };

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
}

pub fn apply_player_movement(
    mut commands: Commands,
    query: Query<(&mut TileCoordinates, Option<&mut Movement>, Entity), With<Player>>,
    level: Res<LevelResource>,
    timer: Res<Time>,
) {
    for (mut tile, movement, entity) in query {
        match movement {
            Some(mut movement) => {
                let animation_percentage = movement.animation_percentage;
                movement.animation_percentage += movement.movement_speed * timer.delta_secs();

                // If animation is finished, set the actual coordinates and stop animating.
                if animation_percentage >= 1.0 {
                    tile.x += movement.offset.x as isize;
                    tile.y += movement.offset.y as isize;
                    tile.z += movement.offset.z as isize;

                    movement.animation_percentage = 0.0;
                    commands.entity(entity).remove::<Movement>();

                    // Only trigger this event if it was not falling
                    if movement.offset.y == 0.0
                        && movement.offset != Vec3::ZERO
                        && matches!(level.level_state, LevelState::ProcessingPlayerInput)
                    {
                        info!("Triggering player finished event {movement:#?}");
                        commands.trigger(PlayerFinishedMoving {});
                    }
                }
            }
            None => {}
        }
    }
}

pub fn set_transform_based_on_tile_coordinates(
    query: Query<(&mut Transform, &TileCoordinates, Option<&Movement>)>,
) {
    let sqrt3 = 3f32.sqrt();

    for (mut transform, tile, movement) in query {
        // An offset in the z-coordinate will move it to the right and up (visually); we use hexagonal geometry with pointy tops.
        transform.translation.x = ((tile.x as f32) + (tile.z as f32) / 2f32) * sqrt3;
        transform.translation.z = (tile.z as f32) * -1.5;

        // A tile's width is 0.6 so if an object is supposed to be on top of the tile, grant it +0.2 on the z-axis.
        transform.translation.y = 0.8 * tile.y as f32 + if tile.is_on_top { 0.6 } else { 0.0 };

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

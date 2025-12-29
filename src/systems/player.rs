use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;

use crate::components::{
    goal::Goal,
    player::Player,
    tile::Tile,
    tile_coordinates::{MovementDirection, TileCoordinates},
};

const BLOOM_COLOR: LinearRgba = LinearRgba::rgb(1.0, 0.0, 1.0);

pub(crate) fn add_player_bloom(
    mut commands: Commands,
    query: Query<(Entity, &Player)>,
    children: Query<&Children>,
    mesh_materials: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _tile) in &query {
        for descendant in children.iter_descendants(entity) {
            let Ok((id, _material_name)) = mesh_materials.get(descendant) else {
                continue;
            };

            let Some(material) = asset_materials.get_mut(id.id()) else {
                continue;
            };

            if material.emissive != BLOOM_COLOR {
                let mut new_material = material.clone();
                new_material.emissive = BLOOM_COLOR;

                commands
                    .entity(descendant)
                    .insert(MeshMaterial3d(asset_materials.add(new_material)));
            }
        }
    }
}

pub(crate) fn collect_goals(
    mut commands: Commands,
    players: Query<(&Player, &TileCoordinates), Without<Goal>>,
    goals: Query<(&Goal, &TileCoordinates, Entity), Without<Player>>,
) {
    for player in players {
        if player.1.movement_animation_percentage.is_some() {
            // Allow movement to finish first
            continue;
        }

        for goal in goals {
            if player.1.x == goal.1.x && player.1.y == goal.1.y && player.1.z == goal.1.z {
                // TODO: awesome explosive animation?
                commands.entity(goal.2).despawn();
            }
        }
    }
}

pub(crate) fn player_controls(
    players: Query<(&mut Player, &mut TileCoordinates), Without<Tile>>,
    tiles: Query<(&Tile, &TileCoordinates)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    for mut player in players {
        if let Some(value) = player.1.movement_animation_percentage {
            let speed = match player.1.movement_direction {
                Some(MovementDirection::Down) => player.1.falling_speed,
                _ => player.1.movement_speed,
            };

            player.1.movement_animation_percentage =
                Some((value + speed * timer.delta_secs()).clamp(0.0, 1.0));

            // If already moving, then movement cannot be altered
            continue;
        }

        if !tiles.iter().any(|tile| {
            tile.1.is_on_top
                && tile.1.x == player.1.x
                && tile.1.y == player.1.y
                && tile.1.z == player.1.z
        }) {
            // Player is not currently standing. Disable movement and start falling instead!
            player.1.movement_direction = Some(MovementDirection::Down);
            player.1.movement_animation_percentage = Some(0.0);

            continue;
        }

        if keys.just_pressed(KeyCode::KeyA) {
            player.1.movement_direction = Some(MovementDirection::West);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::KeyD) {
            player.1.movement_direction = Some(MovementDirection::East);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::KeyW) {
            player.1.movement_direction = Some(MovementDirection::NorthWest);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::KeyE) {
            player.1.movement_direction = Some(MovementDirection::NorthEast);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::KeyZ) {
            player.1.movement_direction = Some(MovementDirection::SouthWest);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::KeyX) {
            player.1.movement_direction = Some(MovementDirection::SouthEast);
            player.1.movement_animation_percentage = Some(0.0);
        }

        // Abort moving if there is no tile to move to.
        // TODO: Improve this collision detection (walls, other stuff in the future).
        // Movement is currently allowed if there is a tile-top at the destination or below it.
        if let Some(direction) = player.1.movement_direction.clone() {
            let direction_offset = direction.get_tile_coordinate_offset();
            let target_coordinate = (
                player.1.x + direction_offset.0,
                player.1.y + direction_offset.1,
                player.1.z + direction_offset.2,
            );

            if !tiles.iter().any(|tile| {
                tile.1.is_on_top
                    && tile.1.x == target_coordinate.0
                    && tile.1.y <= target_coordinate.1
                    && tile.1.z == target_coordinate.2
            }) {
                // Character can't go here; a tile top must be here or below it.
                player.1.movement_direction = None;
                player.1.movement_animation_percentage = None;
            }
        }
    }
}

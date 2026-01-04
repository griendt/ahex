use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;
use bevy_hanabi::ParticleEffect;

use crate::{
    components::{
        camera::CameraAngle,
        goal::Goal,
        movement::Movement,
        player::Player,
        tile::Tile,
        tile_coordinates::{MovementDirection, TileCoordinates},
    },
    resources::effects::GlobalEffects,
};

const BLOOM_COLOR: LinearRgba = LinearRgba::rgb(1.0, 0.0, 1.0);

pub fn add_player_bloom(
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

pub fn collect_goals(
    mut commands: Commands,
    players: Query<(&Player, &TileCoordinates), Without<Goal>>,
    goals: Query<(&Goal, &TileCoordinates, &Transform, Entity), Without<Player>>,
    effects: Res<GlobalEffects>,
) {
    for player in players {
        if player.1.movement_animation_percentage.is_some() {
            // Allow movement to finish first
            continue;
        }

        for goal in goals {
            if player.1.x == goal.1.x && player.1.y == goal.1.y && player.1.z == goal.1.z {
                commands.entity(goal.3).despawn();
                commands.spawn((
                    ParticleEffect::new(effects.goal_explosion_effect.clone().unwrap()),
                    goal.2.clone(),
                ));
            }
        }
    }
}

pub fn player_controls(
    mut commands: Commands,
    players: Query<(&Player, &TileCoordinates, Option<&mut Movement>, Entity), Without<Tile>>,
    tiles: Query<(&Tile, &TileCoordinates)>,
    camera: Single<&CameraAngle>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    for player in players {
        if player.2.is_some() {
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
            commands.entity(player.3).insert(Movement {
                animation_percentage: 0.0,
                movement_speed: player.1.falling_speed,
                offset: Vec3::new(0.0, 1.0, 0.0),
            });

            continue;
        }

        let mut movement_direction: Option<MovementDirection> = None;

        if keys.just_pressed(KeyCode::KeyA) {
            movement_direction = Some(MovementDirection::West);
        }
        if keys.just_pressed(KeyCode::KeyD) {
            movement_direction = Some(MovementDirection::East);
        }
        if keys.just_pressed(KeyCode::KeyW) {
            movement_direction = Some(MovementDirection::NorthWest);
        }
        if keys.just_pressed(KeyCode::KeyE) {
            movement_direction = Some(MovementDirection::NorthEast);
        }
        if keys.just_pressed(KeyCode::KeyZ) {
            movement_direction = Some(MovementDirection::SouthWest);
        }
        if keys.just_pressed(KeyCode::KeyX) {
            movement_direction = Some(MovementDirection::SouthEast);
        }

        match movement_direction {
            Some(direction) => {
                let offset = direction
                    .rotate_y(camera.total_6th_rotations)
                    .get_tile_coordinate_offset();

                let destination_tile = (
                    player.1.x + offset.x as isize,
                    player.1.y + offset.y as isize,
                    player.1.z + offset.z as isize,
                );

                // Only move the player if there is a destination tile
                // at the destination (or below it). Otherwise the player
                // could fall off the island.
                if tiles.iter().any(|tile| {
                    tile.1.is_on_top
                        && tile.1.x == destination_tile.0
                        && tile.1.y <= destination_tile.1
                        && tile.1.z == destination_tile.2
                }) {
                    commands.entity(player.3).insert(Movement {
                        animation_percentage: 0.0,
                        movement_speed: player.1.movement_speed,
                        offset: offset,
                    });
                }
            }
            None => {}
        };
    }
}

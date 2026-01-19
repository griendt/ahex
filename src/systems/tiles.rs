use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;
use bevy_polyline::prelude::{
    Polyline, PolylineBundle, PolylineHandle, PolylineMaterial, PolylineMaterialHandle,
};

use crate::{
    components::{
        movement::Movement,
        player::{Player, PlayerFinishedMoving, PlayerStartedMoving},
        tile::{
            Carriable, HasGravity, IcyTile, MovementMap, ShouldRenderMovementMapPolylines, Tile,
        },
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

pub fn patch_icy_tile_texture(
    query: Query<(Entity, &IcyTile)>,
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

            // info!("Got icy tile, the refletance is {:?}", material);
            // material.base_color = Color::linear_rgba(1.0, 1.0, 1.0, 0.2);
            material.specular_transmission = -50.;
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

pub fn on_player_started_moving(_event: On<PlayerStartedMoving>, mut level: ResMut<LevelResource>) {
    level.level_state = LevelState::ProcessingPlayerInput;
}

pub fn on_players_finished_moving(
    _event: On<PlayerFinishedMoving>,
    query: Query<
        (
            &TileCoordinates,
            &mut MovementMap,
            Option<&Movement>,
            Entity,
        ),
        Without<Carriable>,
    >,
    carriables: Query<(&TileCoordinates, &Carriable, Entity), Without<Movement>>,
    mut level: ResMut<LevelResource>,
    mut commands: Commands,
) {
    level.level_state = LevelState::ProcessingLevelEffects;

    // Apply movement maps
    for (tile, mut movement_map, existing_movement, entity) in query {
        if movement_map.map.is_empty() || existing_movement.is_some() {
            continue;
        }

        let offset = movement_map.map[movement_map.index % movement_map.map.len()];

        let movement = Movement {
            offset: Vec3::new(offset.0 as f32, offset.1 as f32, offset.2 as f32),
            movement_speed: tile.movement_speed,
            animation_percentage: 0.0,
        };

        commands.entity(entity).insert(movement.clone());
        movement_map.index += 1;

        for (carriable_coordinates, _carriable, carriable_entity) in carriables {
            if carriable_coordinates.x == tile.x
                && carriable_coordinates.y == tile.y
                && carriable_coordinates.z == tile.z
            {
                commands.entity(carriable_entity).insert(movement.clone());
            }
        }
    }
}

pub fn apply_movement(
    mut commands: Commands,
    moving_objects: Query<(
        &mut TileCoordinates,
        &mut Movement,
        Option<&HasGravity>,
        Option<&Player>,
        Entity,
    )>,
    still_objects: Query<(&TileCoordinates, Option<&Tile>, Option<&IcyTile>), Without<Movement>>,
    mut level: ResMut<LevelResource>,
    timer: Res<Time>,
) {
    let mut all_moving_players_finished_moving = true;
    let mut all_moving_objects_finished_moving = true;

    for (mut tile_coordinates, mut movement, has_gravity, is_player, entity) in moving_objects {
        let animation_percentage = movement.animation_percentage;
        movement.animation_percentage = (movement.animation_percentage
            + movement.movement_speed * timer.delta_secs())
        .clamp(0., 1.);

        // If animation is finished, set the actual coordinates and stop animating.
        if animation_percentage >= 1.0 {
            tile_coordinates.x += movement.offset.x as isize;
            tile_coordinates.y += movement.offset.y as isize;
            tile_coordinates.z += movement.offset.z as isize;

            movement.animation_percentage -= 1.0;

            let next_tile = (
                tile_coordinates.x + movement.offset.x as isize,
                tile_coordinates.y + movement.offset.y as isize,
                tile_coordinates.z + movement.offset.z as isize,
            );

            // If the object is landing on an icy tile, and it can continue onwards, then make it slide onward.
            if still_objects.iter().any(|object| {
                object.2.is_some()
                    && object.0.x == tile_coordinates.x
                    && object.0.y == tile_coordinates.y
                    && object.0.z == tile_coordinates.z
            }) && !still_objects.iter().any(|object| {
                object.1.is_some()
                    && object.0.x == next_tile.0
                    && object.0.y == next_tile.1 + 1
                    && object.0.z == next_tile.2
            }) {
                info!("Object is on an icy object");

                all_moving_objects_finished_moving = false;

                if is_player.is_some() {
                    all_moving_players_finished_moving = false;
                }
                continue;
            }

            commands.entity(entity).remove::<Movement>();

            // If there is no tile at the destination tile, the object is going to fall.
            // NOTE: We only do this in the `ProcessingPlayerInput` phase right now, because
            // otherwise there is a race condition with moving tiles that finish moving and have their coordinates updated.
            // This check should technically come _after_ those tiles are done moving.
            if has_gravity.is_some()
                && !still_objects.iter().any(|object| {
                    object.0.is_on_top
                        && object.0.x == tile_coordinates.x
                        && object.0.y == tile_coordinates.y
                        && object.0.z == tile_coordinates.z
                })
                && matches!(level.level_state, LevelState::ProcessingPlayerInput)
            {
                commands.entity(entity).insert(Movement {
                    offset: Vec3::new(0., -1., 0.),
                    movement_speed: tile_coordinates.falling_speed,
                    animation_percentage: 0.0,
                });

                all_moving_objects_finished_moving = false;

                if is_player.is_some() {
                    all_moving_players_finished_moving = false;
                }
            }
        } else {
            all_moving_objects_finished_moving = false;

            if is_player.is_some() {
                all_moving_players_finished_moving = false;
            }
        }
    }

    if all_moving_players_finished_moving
        && matches!(level.level_state, LevelState::ProcessingPlayerInput)
    {
        commands.trigger(PlayerFinishedMoving {});
    }

    if all_moving_objects_finished_moving
        && matches!(level.level_state, LevelState::ProcessingLevelEffects)
    {
        level.level_state = LevelState::WaitingForPlayerInput;
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

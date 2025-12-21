use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;

use crate::components::{
    player::Player,
    tile_coordinates::{MovementDirection, TileCoordinates},
};

const BLOOM_COLOR: LinearRgba = LinearRgba::rgb(3.0, 1.0, 1.0);

pub(crate) fn add_player_bloom(
    mut commands: Commands,
    query: Query<(Entity, &Player)>,
    children: Query<&Children>,
    mesh_materials: Query<(&MeshMaterial3d<StandardMaterial>, &GltfMaterialName)>,
    mut asset_materials: ResMut<Assets<StandardMaterial>>,
) {
    for (entity, _tile) in &query {
        for descendant in children.iter_descendants(entity) {
            let Ok((id, material_name)) = mesh_materials.get(descendant) else {
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

pub(crate) fn player_controls(
    players: Query<(&mut Player, &mut TileCoordinates)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    for mut player in players {
        if let Some(value) = player.1.movement_animation_percentage {
            player.1.movement_animation_percentage =
                Some((value + player.1.movement_speed * timer.delta_secs()).clamp(0.0, 1.0));

            // If already moving, then movement cannot be altered
            continue;
        }

        // TODO: Check if target field is not occupied with anything solid.
        if keys.pressed(KeyCode::KeyA) {
            player.1.movement_direction = Some(MovementDirection::West);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.pressed(KeyCode::KeyD) {
            player.1.movement_direction = Some(MovementDirection::East);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.pressed(KeyCode::KeyW) {
            player.1.movement_direction = Some(MovementDirection::NorthWest);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.pressed(KeyCode::KeyE) {
            player.1.movement_direction = Some(MovementDirection::NorthEast);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.pressed(KeyCode::KeyZ) {
            player.1.movement_direction = Some(MovementDirection::SouthWest);
            player.1.movement_animation_percentage = Some(0.0);
        }
        if keys.pressed(KeyCode::KeyX) {
            player.1.movement_direction = Some(MovementDirection::SouthEast);
            player.1.movement_animation_percentage = Some(0.0);
        }
    }
}

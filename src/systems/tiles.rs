use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;

use crate::components::{
    tile::Tile,
    tile_coordinates::{MovementDirection, TileCoordinates},
};

pub(crate) fn colorize_tiles(
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

                    commands
                        .entity(descendant)
                        .insert(MeshMaterial3d(asset_materials.add(new_material)));
                }
            }
        }
    }
}

pub(crate) fn set_tile_transform(query: Query<(&mut Transform, &mut TileCoordinates)>) {
    let sqrt3 = 3f32.sqrt();

    for (mut transform, mut tile) in query {
        // If the entity is currently moving, offset by the appropriate amount
        let offset = tile
            .movement_direction
            .clone()
            .map(|direction| direction.get_tile_coordinate_offset())
            .unwrap_or_default();

        let animation_percentage = tile.movement_animation_percentage.unwrap_or_default();

        // If animation is finished, set the actual coordinates and stop animating.
        if animation_percentage >= 1.0 {
            tile.x += offset.0;
            tile.y += offset.1;
            tile.z += offset.2;
            tile.movement_animation_percentage = None;
            tile.movement_direction = None;
        }

        // An offset in the y-coordinate will move it to the right and up; we use hexagonal geometry with pointy tops.
        transform.translation.x = ((tile.x as f32) + (tile.y as f32) / 2f32) * sqrt3;
        transform.translation.y = (tile.y as f32) * 1.5;

        // A tile's width is 0.2 so if an object is supposed to be on top of the tile, grant it +0.2 on the z-axis.
        transform.translation.z = 0.8 * tile.z as f32 + if tile.is_on_top { 0.2 } else { 0.0 };

        // Display the entity percentually towards the destination coordinates, if animating.
        if animation_percentage < 1.0 {
            transform.translation.x +=
                animation_percentage * (sqrt3 * (offset.0 as f32 + offset.1 as f32 / 2f32) as f32);
            transform.translation.y += animation_percentage * (offset.1 as f32 * 1.5);
            transform.translation.z += animation_percentage * (0.8 * offset.2 as f32);
        }
    }
}

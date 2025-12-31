use bevy::prelude::*;
use bevy_gltf::GltfMaterialName;

use crate::components::goal::Goal;

const BLOOM_COLOR: LinearRgba = LinearRgba::rgb(1.0, 1.0, 0.0);

pub fn rotate_goal(query: Query<&mut Transform, With<Goal>>, timer: Res<Time>) {
    for mut goal in query {
        goal.rotate_local_x(timer.delta_secs());
        goal.rotate_local_y(timer.delta_secs() / 2.0);
        goal.rotate_local_z(timer.delta_secs() / 4.0);
    }
}

pub fn add_goal_bloom(
    mut commands: Commands,
    query: Query<(Entity, &Goal)>,
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

pub fn vary_goal_intensity(query: Query<&mut PointLight, With<Goal>>, timer: Res<Time>) {
    for mut light in query {
        light.intensity = 300_000.0 + 200_000.0 * timer.elapsed_secs().sin();
    }
}

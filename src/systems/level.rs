use bevy::prelude::*;

use crate::components::level::{LevelEntityMarker, LevelParser};

pub fn build_level(
    mut commands: Commands,
    level: Single<&LevelParser>,
    asset_server: Res<AssetServer>,
) {
    level.render_level(&mut commands, &asset_server);
}

pub fn restart_level(
    mut commands: Commands,
    level: Single<&LevelParser>,
    entities: Query<(&LevelEntityMarker, Entity), Without<LevelParser>>,
    asset_server: Res<AssetServer>,
    keys: Res<ButtonInput<KeyCode>>,
) {
    if keys.just_pressed(KeyCode::Backspace) {
        for entity in entities {
            commands.entity(entity.1).despawn();
        }

        level.render_level(&mut commands, &asset_server);
    }
}

use bevy::prelude::*;

use crate::components::{
    player::Player,
    tile_coordinates::{MovementDirection, TileCoordinates},
};

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

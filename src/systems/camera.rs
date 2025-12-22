use std::f32::consts::TAU;

use bevy::prelude::*;

use crate::components::camera::{CameraAngle, RotationDirection};

pub(crate) fn move_camera(
    mut camera: Single<(&mut Transform, &mut CameraAngle)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    if let Some(value) = camera.1.rotation_animation_percentage {
        camera.1.rotation_animation_percentage =
            Some((value + camera.1.rotation_speed * timer.delta_secs()).clamp(0.0, 1.0));

        if camera.1.rotation_animation_percentage.unwrap_or_default() >= 1.0 {
            camera.1.total_6th_rotations += match camera.1.rotation_direction {
                Some(RotationDirection::Clockwise) => 1,
                Some(RotationDirection::CounterClockwise) => -1,
                None => 0,
            };

            camera.1.rotation_animation_percentage = None;
            camera.1.rotation_direction = None;
        }
    } else {
        if keys.just_pressed(KeyCode::ArrowLeft) {
            camera.1.rotation_direction = Some(RotationDirection::Clockwise);
            camera.1.rotation_animation_percentage = Some(0.0);
        }
        if keys.just_pressed(KeyCode::ArrowRight) {
            camera.1.rotation_direction = Some(RotationDirection::CounterClockwise);
            camera.1.rotation_animation_percentage = Some(0.0);
        }
    }

    let rotation_radius = 10.0;
    let angle = TAU
        * ((camera.1.total_6th_rotations as f32
            + (camera.1.rotation_animation_percentage.unwrap_or_default()
                * match camera.1.rotation_direction {
                    Some(RotationDirection::Clockwise) => 1.0,
                    Some(RotationDirection::CounterClockwise) => -1.0,
                    None => 0.0,
                })
            + 1.5)
            / 6.0);

    let new_x = rotation_radius * angle.cos();
    let new_z = rotation_radius * angle.sin();

    camera.0.translation.x = new_x;
    camera.0.translation.z = new_z;
    camera.0.look_at(Vec3::ZERO, Vec3::Y);
}

use bevy::prelude::*;

pub(crate) fn move_camera(
    mut camera: Single<(&mut Transform, &mut Camera3d)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    if keys.pressed(KeyCode::ArrowLeft) {
        camera
            .0
            .rotate_around(Vec3::ZERO, Quat::from_rotation_y(3.0 * timer.delta_secs()));
    }
    if keys.pressed(KeyCode::ArrowRight) {
        camera
            .0
            .rotate_around(Vec3::ZERO, Quat::from_rotation_y(-3.0 * timer.delta_secs()));
    }

    camera.0.look_at(Vec3::new(0.0, 0.0, 0.0), Vec3::Y);
}

use bevy::prelude::*;

pub(crate) fn move_camera(
    mut camera: Single<(&mut Transform, &mut Camera3d)>,
    keys: Res<ButtonInput<KeyCode>>,
    timer: Res<Time>,
) {
    if keys.pressed(KeyCode::ArrowLeft) {
        camera.0.translation.x += 10.0 * timer.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowRight) {
        camera.0.translation.x -= 10.0 * timer.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowUp) {
        camera.0.translation.y += 5.0 * timer.delta_secs();
        camera.0.translation.z -= 15.0 * timer.delta_secs();
    }
    if keys.pressed(KeyCode::ArrowDown) {
        camera.0.translation.y -= 5.0 * timer.delta_secs();
        camera.0.translation.z += 15.0 * timer.delta_secs();
    }

    let camera_x = camera.0.translation.x.clone();
    camera.0.look_at(Vec3::new(camera_x, 0.0, 0.0), Vec3::Y);
}

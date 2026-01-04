use std::f32::consts::TAU;

use bevy::{
    color::Color,
    ecs::system::{Commands, Res, Single},
    light::DirectionalLight,
    math::Vec3,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::components::lighting::Sun;

pub fn create_the_sun(mut commands: Commands, sun: Option<Single<&Sun>>) {
    if sun.is_some() {
        return;
    }

    commands.spawn((
        Sun {
            time_of_day: 0.45,
            seconds_per_day: 240,
        },
        DirectionalLight {
            shadows_enabled: true,
            soft_shadow_size: None,
            affects_lightmapped_mesh_diffuse: false,
            illuminance: 2000.0,
            color: Color::srgb(1.0, 0.0, 0.0),
            ..default()
        },
        Transform::from_xyz(-60.0, 100.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn update_the_sun(
    mut sun: Single<(&mut Sun, &mut DirectionalLight, &mut Transform)>,
    timer: Res<Time>,
) {
    let day_fraction_passed = timer.delta_secs() / sun.0.seconds_per_day as f32;

    sun.0.time_of_day += day_fraction_passed;
    sun.0.time_of_day = sun.0.time_of_day.fract();

    let redness = (TAU * sun.0.time_of_day).sin();

    sun.1.color = Color::srgb(0.5 + redness / 2.0, 1.0 - redness, 1.0 - redness);
    sun.1.illuminance = 2000.0 - 1500.0 * (TAU * sun.0.time_of_day).cos();
    sun.2.rotate_y(-TAU * day_fraction_passed);
    sun.2.translation.y = -100.0 + 200.0 * sun.0.time_of_day;
}

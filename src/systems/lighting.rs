use std::f32::{consts::TAU};

use bevy::{
    color::Color,
    ecs::system::{Commands, Res, Single},
    light::{DirectionalLight},
    math::Vec3,
    time::Time,
    transform::components::Transform,
    utils::default,
};

use crate::{components::lighting::Sun, resources::settings::Settings};

pub fn create_the_sun(mut commands: Commands, sun: Option<Single<&Sun>>, settings: Res<Settings>) {
    if sun.is_some() {
        return;
    }

    commands.spawn((
        Sun {
            time_of_day: settings.display.sun.initial_time_of_day,
            seconds_per_day: settings.display.sun.seconds_per_day,
        },
        DirectionalLight {
            shadows_enabled: settings.display.sun.shadows_enabled,
            soft_shadow_size: None,
            affects_lightmapped_mesh_diffuse: true,
            illuminance: settings.display.sun.illuminance,
            color: Color::WHITE,
            ..default()
        },
        Transform::from_xyz(5.0, 10.0, -5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}

pub fn update_the_sun(
    mut sun: Single<(&mut Sun, &mut DirectionalLight, &mut Transform)>,
    timer: Res<Time>,
    settings: Res<Settings>,
) {
    let day_fraction_passed = timer.delta_secs() / sun.0.seconds_per_day as f32;

    sun.0.time_of_day += day_fraction_passed;
    sun.0.time_of_day = sun.0.time_of_day.fract();


    sun.2.translation.x = -5.0 * (TAU * sun.0.time_of_day).cos();
    sun.2.translation.y = settings.display.sun.height - settings.display.sun.height_variation * (TAU * sun.0.time_of_day).cos();
    sun.2.translation.z = 5.0 * (TAU * sun.0.time_of_day).sin();
    sun.2.look_at(Vec3::ZERO, Vec3::Y);

    sun.1.illuminance = match sun.2.translation.y < 0.0 {
        true => 0.0,
        false =>  settings.display.sun.illuminance
    };

}

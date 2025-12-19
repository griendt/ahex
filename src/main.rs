use bevy::{light::DirectionalLightShadowMap, prelude::*};

use crate::systems::{
    camera::move_camera,
    setup::setup,
    tiles::{colorize_tiles, set_tile_transform},
};

mod components;
mod systems;

fn main() {
    App::new()
        .insert_resource(DirectionalLightShadowMap { size: 8192 })
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_systems(Startup, setup)
        .add_systems(Update, (move_camera, colorize_tiles, set_tile_transform))
        .run();
}

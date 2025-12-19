use bevy::{color::Color, ecs::component::Component};

#[derive(Component)]
pub(crate) struct Tile {
    pub(crate) color: Color,
}

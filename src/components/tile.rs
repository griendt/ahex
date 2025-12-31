use bevy::{color::Color, ecs::component::Component};

#[derive(Component)]
pub struct Tile {
    pub color: Color,
}

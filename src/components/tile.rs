use bevy::{color::Color, ecs::component::Component};

#[derive(Component)]
pub struct Tile {
    pub color: Color,
}

#[derive(Component)]
pub struct IcyTile;

#[derive(Component)]
pub struct HasGravity;

#[derive(Component)]
pub struct Carriable;

#[derive(Component)]
pub struct MovementMap {
    pub map: Vec<(isize, isize, isize)>,
    pub index: usize,
}

#[derive(Component)]
pub struct ShouldRenderMovementMapPolylines;

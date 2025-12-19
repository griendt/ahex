use bevy::ecs::component::Component;

#[derive(Component)]
pub(crate) struct TileCoordinates {
    pub(crate) y: isize,
    pub(crate) x: isize,
    pub(crate) z: isize,
    pub(crate) is_on_top: bool,
}

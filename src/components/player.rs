use bevy::ecs::{component::Component, event::Event};

#[derive(Component)]
pub struct Player {}

#[derive(Event)]
pub struct PlayerFinishedMoving {}

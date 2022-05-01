use bevy::prelude::Component;

#[derive(Component, Clone, Copy)]
pub enum TileType {
    Obstacle,
    Room,
    Started,
}

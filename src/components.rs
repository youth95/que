use bevy::prelude::*;

#[derive(Component, Clone, Copy)]
pub enum TileType {
    Obstacle,
    Room,
    Started,
}

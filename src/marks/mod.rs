use bevy::prelude::Component;

use crate::regions::Tile;

pub mod region_status;
pub mod region_type;

#[derive(Component, Clone)]
pub struct Region(pub Tile);

#[derive(Component)]
pub struct IDText;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct NPC;

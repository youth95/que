use bevy::prelude::Component;

pub mod region_status;
pub mod region_type;

#[derive(Component, Clone)]
pub struct RegionRect;

#[derive(Component, Clone, Copy)]
pub struct RegionId(pub u64);

#[derive(Component)]
pub struct IDText;

#[derive(Component)]
pub struct Player;

#[derive(Component, Clone)]
pub struct Enemy {
    pub name: String,
    pub max_hp: u64,
    pub cur_hp: i64,
}

#[derive(Component)]
pub struct EnemyMark;
#[derive(Component)]
pub struct HPText;

#[derive(Component)]
pub struct NPC;

#[derive(Component, Clone, Copy, PartialEq)]
pub enum RegionStatus {
    Found,
    Visited,
    Mist,
}

#[derive(Component, Clone, Copy)]
pub struct Visited;

#[derive(Component, Clone, Copy)]
pub struct Mist;

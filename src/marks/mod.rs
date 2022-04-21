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
pub struct EnemyText;

#[derive(Component)]
pub struct EnemyMark;
#[derive(Component, Clone)]
pub struct EnemyStatus {
    pub name: String,
    pub atk: i64,
    pub def: i64,
    pub max_hp: u64,
    pub cur_hp: i64,
}

#[derive(Component, Clone)]
pub struct EnemyLabel {
    pub name: String,
    pub intro: String,
    pub image_label: String,
}

#[derive(Component)]
pub struct HPColor;

#[derive(Component)]
pub struct NPC;

#[derive(Component, Clone, Copy, PartialEq, Debug)]
pub enum RegionStatus {
    Found,
    Visited,
    Mist,
}

#[derive(Component, Clone, Copy)]
pub struct Visited;

#[derive(Component, Clone, Copy)]
pub struct Mist;

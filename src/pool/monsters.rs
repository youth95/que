use serde::{Deserialize, Serialize};

use super::Pool;

#[derive(Debug, Deserialize, Serialize)]
pub struct Monster {
    pub name: String,
    pub intro: String,
    pub icon: String,
    pub atk: i64,
    pub def: i64,
    pub hp: u64,
    pub image_label: String,
}

pub fn get_monsters_pool() -> Pool<Monster> {
    let config = include_str!("../../assets/pool/monsters.ron");
    ron::from_str(config).unwrap()
}

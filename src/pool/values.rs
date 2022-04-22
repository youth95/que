use bevy::prelude::Component;
use serde::{Deserialize, Serialize};

use crate::rng::RAND;

use super::Pool;

#[derive(Debug, Deserialize, Serialize, Component, Clone)]
pub struct Value {
    pub name: String,
    pub intro: String,
    pub image_label: String,
    pub values: Vec<KeyValue>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum KeyValue {
    PlayerCurrentHp(Val),
    PlayerMaxHp(Val),
    PlayerAtk(Val),
    PlayerDef(Val),
    PlayerGold(Val),
}

impl Val {
    pub fn to_i64(&self) -> i64 {
        match self {
            Val::Fixed(v) => *v,
            Val::Float(min, max) => RAND.lock().unwrap().random_range_i64(*min, *max).1,
        }
    }

    pub fn to_str(&self) -> String {
        match self {
            Val::Fixed(v) => format!("{}{}", if *v >= 0 { "+" } else { "-" }, v.abs()),
            Val::Float(min, max) => format!("{} ~ {}", min, max),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub enum Val {
    Fixed(i64),
    Float(i64, i64), // min max
}

pub fn get_values_pool() -> Pool<Value> {
    let config = include_str!("../../assets/pool/values.ron");
    ron::from_str(config).unwrap()
}

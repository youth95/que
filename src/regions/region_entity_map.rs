use bevy::{
    prelude::{Entity, Resource},
    utils::HashMap,
};

#[derive(Resource, Default)]
pub struct RegionEntityMap(pub HashMap<u64, Entity>);

#[derive(Resource)]
pub enum CurrentOverRegion {
    None,
    Region(u64),
}

impl Default for CurrentOverRegion {
    fn default() -> Self {
        CurrentOverRegion::None
    }
}

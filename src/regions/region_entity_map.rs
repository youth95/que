use bevy::{prelude::Entity, utils::HashMap};

#[derive(Default)]
pub struct RegionEntityMap(pub HashMap<u64, Entity>);

pub enum CurrentOverRegion {
    None,
    Region(u64),
}

impl Default for CurrentOverRegion {
    fn default() -> Self {
        CurrentOverRegion::None
    }
}

pub mod events;
mod manager;
mod pure;
mod region_entity_map;
mod renderer;

pub use self::events::{ChangeEnemyHpEvent, ChangeRegionStatusEvent, RegionClickEvent};
pub use self::manager::Regions;
use self::pure::RegionPurePlugin;
pub use self::region_entity_map::{CurrentOverRegion, RegionEntityMap};
use self::renderer::RegionRenderPlugin;
use bevy::prelude::Plugin;
pub use pure::RegionMark;
pub use renderer::WorldMouse;

pub struct RegionPlugin;

impl Plugin for RegionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RegionPurePlugin)
            .add_plugin(RegionRenderPlugin);
    }
}

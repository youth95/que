mod events;
mod manager;
mod pure;
mod renderer;

use bevy::prelude::Plugin;

pub use self::events::{ChangeEnemyHpEvent, ChangeRegionStatusEvent, TriggerRegionEvent};
pub use self::manager::Regions;
use self::pure::RegionPurePlugin;
use self::renderer::RegionRenderPlugin;

pub struct RegionPlugin;

impl Plugin for RegionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(RegionPurePlugin)
            .add_plugin(RegionRenderPlugin);
    }
}

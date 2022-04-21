#[macro_use]
pub extern crate lazy_static;
pub extern crate serde;

mod camera;
pub mod components;
// mod debugger;
pub mod marks;
mod player;
pub mod pool;
pub mod regions;
pub mod rng;

use bevy::prelude::Plugin;
use bevy_kira_audio::AudioPlugin;
pub use camera::CameraPlugin;
// pub use debugger::DebuggerPlugin;
use player::PlayerPlugin;
use regions::RegionPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app
            // .add_plugin(DebuggerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(RegionPlugin)
            .add_plugin(PlayerPlugin);
    }
}

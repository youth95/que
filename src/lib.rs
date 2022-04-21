#[macro_use]
pub extern crate lazy_static;
pub extern crate serde;

mod camera;
pub mod components;
// mod debugger;
mod assets;
pub mod marks;
mod player;
pub mod pool;
pub mod regions;
pub mod rng;

pub use assets::AudioAssets;
use assets::MonsterImageAssets;
use bevy::prelude::Plugin;
use bevy_kira_audio::AudioPlugin;
pub use camera::CameraPlugin;
// pub use debugger::DebuggerPlugin;
use player::PlayerPlugin;
use regions::RegionPlugin;

use bevy_asset_loader::AssetLoader;

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStage {
    Loading,
    Main,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        AssetLoader::new(GameStage::Loading)
            .continue_to_state(GameStage::Main)
            .with_collection::<MonsterImageAssets>()
            .with_collection::<AudioAssets>()
            .build(app);

        app.add_state(GameStage::Loading)
            .add_plugin(CameraPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(RegionPlugin)
            .add_plugin(PlayerPlugin);
    }
}

#[macro_use]
pub extern crate lazy_static;
pub extern crate serde;
pub extern crate bevy;

mod camera;
pub mod components;
// mod debugger;
mod assets;
mod game_over;
pub mod marks;
pub mod panel;
mod player;
mod player_skill;
pub mod pool;
pub mod regions;
pub mod rng;

pub use assets::AudioAssets;
use assets::{FontAssets, MonsterImageAssets, UIImageAssets};
use bevy::{diagnostic::LogDiagnosticsPlugin, prelude::*};
use bevy_kira_audio::AudioPlugin;
use bevy_ninepatch::*;
pub use camera::CameraPlugin;
use game_over::GameOverPlugin;
use panel::PanelPlugin;
pub use player::PlayerStatusType;
// pub use debugger::DebuggerPlugin;
use player::PlayerPlugin;
use player_skill::PlayerSkillPlugin;
use regions::RegionPlugin;
use bevy_embedded_assets::EmbeddedAssetPlugin;


use bevy_asset_loader::AssetLoader;

pub struct GamePlugin;

#[derive(Clone, Eq, PartialEq, Debug, Hash)]
pub enum GameStage {
    Loading,
    Main,
    GameOver,
}

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        AssetLoader::new(GameStage::Loading)
            .continue_to_state(GameStage::Main)
            .with_collection::<MonsterImageAssets>()
            .with_collection::<FontAssets>()
            .with_collection::<AudioAssets>()
            .with_collection::<UIImageAssets>()
            .build(app);

        app.add_state(GameStage::Loading)
            .add_plugin(NinePatchPlugin::<()>::default())
            .add_plugin(PanelPlugin)
            .add_plugin(GameOverPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AudioPlugin)
            .add_plugin(RegionPlugin)
            .add_plugin(PlayerPlugin)
            .add_plugin(PlayerSkillPlugin);
    }
}

pub fn app() -> App {
    let mut app = App::new();
    app.insert_resource(WindowDescriptor {
        title: "䧿".to_string(),
        width: 1024.,
        height: 768.,
        ..Default::default()
    })
    .add_plugins_with(DefaultPlugins, |group| {
        group.add_before::<bevy::asset::AssetPlugin, _>(EmbeddedAssetPlugin)
    })
    .add_plugin(LogDiagnosticsPlugin::default())
    .add_plugin(GamePlugin);

    #[cfg(not(target_arch = "wasm32"))]
    {
        app.add_plugin(bevy::diagnostic::FrameTimeDiagnosticsPlugin);
    }
    // #[cfg(target_arch = "wasm32")]
    // {
    //     app.add_plugin(bevy_web_resizer::Plugin);
    // }
    app
}

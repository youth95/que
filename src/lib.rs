#[macro_use]
pub extern crate lazy_static;
pub extern crate serde;

mod camera;
pub mod components;
mod debugger;
pub mod marks;
pub mod pool;
pub mod regions;
pub mod rng;

use bevy::prelude::Plugin;
use bevy_kira_audio::AudioPlugin;
pub use camera::CameraPlugin;
pub use debugger::DebuggerPlugin;
use regions::{
    atk_monster, mouse_interaction, region_rect_color_system, spawn_tiles_sprite_system,
    change_region_status_system, update_enemy_hp_text_system, visit_region, ChangeEnemyHpEvent,
    ChangeRegionStatusEvent, Regions, TriggerRegionEvent,
};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_plugin(DebuggerPlugin)
            .add_plugin(CameraPlugin)
            .add_plugin(AudioPlugin)
            .add_event::<TriggerRegionEvent>()
            .add_event::<ChangeEnemyHpEvent>()
            .add_event::<ChangeRegionStatusEvent>()
            .init_resource::<Regions>()
            .add_startup_system(spawn_tiles_sprite_system)
            .add_system(mouse_interaction)
            .add_system(atk_monster)
            .add_system(visit_region)
            .add_system(update_enemy_hp_text_system)
            .add_system(change_region_status_system)
            .add_system(region_rect_color_system);
    }
}

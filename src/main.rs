use bevy::prelude::*;
use que::{
    regions::{
        region_rect_color_system, spawn_tiles_sprite_system, trigger_region_system, Regions,
        TriggerRegionEvent,
    },
    CameraPlugin, DebuggerPlugin,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebuggerPlugin)
        .add_plugin(CameraPlugin)
        .add_event::<TriggerRegionEvent>()
        .insert_resource(WindowDescriptor {
            title: "䧿".to_string(),
            width: 1024.,
            height: 768.,
            ..Default::default()
        })
        .init_resource::<Regions>()
        .add_startup_system(spawn_tiles_sprite_system)
        .add_system(trigger_region_system)
        .add_system(region_rect_color_system)
        .run();
}

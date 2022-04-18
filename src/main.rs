use bevy::prelude::*;
use que::{
    regions::{spawn_tiles_sprite_system, Regions},
    CameraPlugin, DebuggerPlugin,
};
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(DebuggerPlugin)
        .add_plugin(CameraPlugin)
        .insert_resource(WindowDescriptor {
            title: "䧿".to_string(),
            ..Default::default()
        })
        .init_resource::<Regions>()
        .add_startup_system(spawn_tiles_sprite_system)
        .run();
}

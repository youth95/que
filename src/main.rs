use bevy::prelude::*;
use que::GamePlugin;
fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "䧿".to_string(),
            width: 1024.,
            height: 768.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(GamePlugin)
        .run();
}

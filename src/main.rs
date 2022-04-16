use bevy::prelude::*;
use que::{regions::Regions, spawn_regions_system::spawn_tiles_sprite_system};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .init_resource::<Regions>()
        .add_startup_system(spawn_tiles_sprite_system)
        .run();
}

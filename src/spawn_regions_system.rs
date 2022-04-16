use crate::pool::Pool;
use bevy::prelude::*;

use crate::pool::terrains::get_plane_orientation_pool;
use crate::pool::terrains::PlaneOrientation;

use super::regions::Regions;

const SIZE: f32 = 32.;
const GAP: f32 = 4.;

const GEN_REGION_ITEMS: u64 = 32 * 32;

pub fn spawn_tiles_sprite_system(mut commands: Commands, mut regions: ResMut<Regions>) {
    let pool: Pool<Vec<PlaneOrientation>> = get_plane_orientation_pool();
    regions.random_generate_tiles(GEN_REGION_ITEMS, &pool);
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for (_, tile) in regions.tiles.iter() {
        let transform = tile.to_transform(SIZE, GAP).unwrap();
        commands
            .spawn()
            .insert_bundle(SpriteBundle::default())
            .insert(transform)
            .insert(Sprite {
                color: Color::BLUE,
                ..Default::default()
            });
    }
}

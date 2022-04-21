use crate::{
    components::TileType,
    marks::{EnemyText, HPColor},
};
use bevy::prelude::*;
pub struct RegionPurePlugin;

use crate::{
    marks::{EnemyMark, EnemyStatus, RegionId, RegionStatus},
    pool::terrains::get_plane_orientation_pool,
};

use super::{
    manager::Tile, ChangeEnemyHpEvent, ChangeRegionStatusEvent, Regions, TriggerRegionEvent,
};

#[derive(Component)]
pub struct RegionMark;

impl Plugin for RegionPurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TriggerRegionEvent>()
            .add_event::<ChangeEnemyHpEvent>()
            .add_event::<ChangeRegionStatusEvent>()
            .init_resource::<Regions>()
            .add_startup_system(spawn_region_system)
            .add_system(atk_monster)
            .add_system(visit_region)
            .add_system(update_enemy_hp_system)
            .add_system(change_region_status_system);
    }
}

const GEN_REGION_ITEMS: u64 = 32 * 32;

pub fn spawn_region_system(mut commands: Commands, mut regions: ResMut<Regions>) {
    let pool = get_plane_orientation_pool();
    regions.random_generate_tiles(GEN_REGION_ITEMS, &pool);
    for (_, region) in regions.tiles.iter() {
        let region_status: RegionStatus = match region.to_tile_type() {
            TileType::Started => RegionStatus::Found,
            _ => RegionStatus::Mist,
        };
        let entity = commands
            .spawn()
            .insert(RegionMark)
            .insert(RegionId(region.id))
            .insert(region_status)
            .id();
        if let TileType::Room = region.to_tile_type() {
            commands.entity(entity).insert(EnemyMark);
        }
    }
}

pub fn atk_monster(
    query: Query<(&RegionId, &RegionStatus), With<EnemyMark>>,
    mut trigger_region_event: EventReader<TriggerRegionEvent>,
    mut change_enemy_hp_event: EventWriter<ChangeEnemyHpEvent>,
    // asset_server: Res<AssetServer>,
    // audio: Res<Audio>,
) {
    for TriggerRegionEvent(id) in trigger_region_event.iter() {
        for (RegionId(region_id), region_status) in query.iter() {
            if region_id == id {
                if *region_status == RegionStatus::Found {
                    change_enemy_hp_event.send(ChangeEnemyHpEvent(*id, -2));
                    // audio.pause();
                    // audio.play(asset_server.load("sounds/dao5.mp3"));
                }
            }
        }
    }
}

pub fn visit_region(
    query: Query<(&RegionId, &RegionStatus), Without<EnemyMark>>,
    mut trigger_region_event: EventReader<TriggerRegionEvent>,
    mut change_region_status_event: EventWriter<ChangeRegionStatusEvent>,
) {
    for TriggerRegionEvent(id) in trigger_region_event.iter() {
        for (RegionId(region_id), region_status) in query.iter() {
            if region_id == id && *region_status == RegionStatus::Found {
                change_region_status_event
                    .send(ChangeRegionStatusEvent(*region_id, RegionStatus::Mist));
            }
        }
    }
}

pub fn update_enemy_hp_system(
    mut query: Query<(&mut EnemyStatus, &RegionId)>,
    mut change_enemy_hp_event: EventReader<ChangeEnemyHpEvent>,
    mut change_region_status_event: EventWriter<ChangeRegionStatusEvent>,
) {
    for ChangeEnemyHpEvent(id, val) in change_enemy_hp_event.iter() {
        for (mut enemy, RegionId(region_id)) in &mut query.iter_mut() {
            if region_id == id && enemy.cur_hp > 0 {
                enemy.cur_hp += val;
                if enemy.cur_hp <= 0 {
                    change_region_status_event
                        .send(ChangeRegionStatusEvent(*id, RegionStatus::Mist));
                }
            }
        }
    }
}

pub fn change_region_status_system(
    mut commands: Commands,
    mut change_region_status_event: EventReader<ChangeRegionStatusEvent>,
    regions: ResMut<Regions>,
    mut sprite_query: Query<(Entity, &RegionId, &RegionStatus), With<RegionMark>>,
    mut hp_text_query: Query<
        (&mut Visibility, &RegionId),
        (With<EnemyStatus>, With<EnemyText>, Without<HPColor>),
    >,
    mut hp_text_color_query: Query<(&mut Visibility, &RegionId), With<HPColor>>,
    // asset_server: Res<AssetServer>,
    // audio: Res<Audio>,
) {
    let mut found_tiles = Vec::<&Tile>::new();
    for ev in change_region_status_event.iter() {
        let ChangeRegionStatusEvent(entity, ..) = ev;
        for (en, RegionId(region_id), status) in sprite_query.iter_mut() {
            match status {
                RegionStatus::Found => {
                    if region_id == entity {
                        commands.entity(en).insert(RegionStatus::Visited);
                        // audio.play(asset_server.load("sounds/click.wav"));
                        if let Some(tile) = regions.tiles.get(region_id) {
                            found_tiles.push(&tile);
                        }
                    }
                }
                _ => (),
            }
        }
    }

    for tile in found_tiles {
        for tile_id in tile.adjacent.clone().into_iter() {
            for (en, RegionId(id), status, ..) in sprite_query.iter_mut() {
                match status {
                    RegionStatus::Mist => {
                        if *id == tile_id {
                            // HPText
                            commands.entity(en).insert(RegionStatus::Found);
                        }
                    }
                    _ => (),
                }
            }
        }
        for tile_id in tile.adjacent.clone().into_iter() {
            for (mut visibility, RegionId(region_id), ..) in hp_text_query.iter_mut() {
                if tile_id == *region_id {
                    visibility.is_visible = true;
                }
            }
        }

        for tile_id in tile.adjacent.clone().into_iter() {
            for (mut visibility, RegionId(region_id), ..) in hp_text_color_query.iter_mut() {
                if tile_id == *region_id {
                    visibility.is_visible = true;
                }
            }
        }
    }

    // hp_text_query
}

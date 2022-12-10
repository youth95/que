use crate::{
    components::TileType,
    marks::EnemyLabel,
    panel::{HasBattlePanel, HasValuePanel},
    player::PlayerStatus,
    pool::{
        monsters::{get_monsters_pool, Monster},
        values::{get_values_pool, Value},
    },
    rng::RAND,
    GameStage,
};
use bevy::prelude::*;
pub struct RegionPurePlugin;

use crate::{
    marks::{EnemyMark, EnemyStatus, RegionId, RegionStatus},
    pool::terrains::get_plane_orientation_pool,
};

use super::{
    events::{
        AtkMonsterWithPlayerSkill, AudioSound, MouseOverEmpty, MouseOverRegionEvent, PlayAudioEvent,
    },
    manager::Tile,
    region_entity_map::{CurrentOverRegion, RegionEntityMap},
    renderer::WorldMouse,
    ChangeEnemyHpEvent, ChangeRegionStatusEvent, RegionClickEvent, Regions,
};

#[derive(Component)]
pub struct RegionMark;

impl Plugin for RegionPurePlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<RegionClickEvent>()
            .add_event::<PlayAudioEvent>()
            .add_event::<ChangeEnemyHpEvent>()
            .add_event::<ChangeRegionStatusEvent>()
            .add_event::<MouseOverEmpty>()
            .add_event::<MouseOverRegionEvent>()
            .add_event::<AtkMonsterWithPlayerSkill>()
            .init_resource::<PlayerStatus>()
            .init_resource::<WorldMouse>()
            .init_resource::<Regions>()
            .init_resource::<RegionEntityMap>()
            .init_resource::<CurrentOverRegion>()
            .add_system_set(SystemSet::on_enter(GameStage::Main).with_system(spawn_region_system))
            .add_system_set(
                SystemSet::on_update(GameStage::Main)
                    // .with_system(click_region_effect)
                    .with_system(atk_monster)
                    .with_system(visit_region)
                    .with_system(visit_value_region)
                    .with_system(update_enemy_hp_system)
                    .with_system(change_region_status_system),
            );
    }
}

const GEN_REGION_ITEMS: u64 = 32 * 32;

impl Monster {
    pub fn to_enemy_status(&self) -> EnemyStatus {
        EnemyStatus {
            name: self.name.clone(),
            atk: self.atk,
            def: self.def,
            max_hp: self.hp,
            cur_hp: self.hp as i64,
        }
    }

    pub fn to_enemy_label(&self) -> EnemyLabel {
        EnemyLabel {
            name: self.name.clone(),
            intro: self.intro.clone(),
            image_label: self.image_label.clone(),
            icon: self.icon.clone(),
        }
    }
}

pub fn spawn_region_system(
    mut commands: Commands,
    mut regions: ResMut<Regions>,
    mut region_entity_map: ResMut<RegionEntityMap>,
) {
    let values_pool = get_values_pool();
    let plane_orientation_pool = get_plane_orientation_pool();
    let monsters_pool = get_monsters_pool();
    regions.clear();
    regions.random_generate_tiles(GEN_REGION_ITEMS, &plane_orientation_pool);

    for (_, region) in regions.tiles.iter() {
        let region_status: RegionStatus = match region.to_tile_type() {
            TileType::Started => RegionStatus::Found,
            _ => RegionStatus::Mist,
        };
        let entity = commands
            .spawn(RegionMark)
            .insert(RegionId(region.id))
            .insert(region_status)
            .id();
        region_entity_map.0.insert(region.id, entity);

        if let TileType::Room = region.to_tile_type() {
            let (_, is_gen_monsters) = RAND.lock().unwrap().random_val_boolean(0.7);
            if is_gen_monsters {
                let monster = monsters_pool.fetch_item();
                commands
                    .entity(entity)
                    .insert(monster.to_enemy_status())
                    .insert(monster.to_enemy_label())
                    .insert(EnemyMark)
                    .insert(HasBattlePanel);
            } else {
                let value = values_pool.fetch_item();
                commands
                    .entity(entity)
                    .insert(value.clone())
                    .insert(HasValuePanel);
            }
        }
    }
}

pub fn atk_monster(
    query: Query<(&RegionId, &RegionStatus)>,
    mut trigger_region_event: EventReader<RegionClickEvent>,
    mut atk_monster_with_player_skill: EventWriter<AtkMonsterWithPlayerSkill>,
) {
    for RegionClickEvent(id) in trigger_region_event.iter() {
        for (RegionId(region_id), region_status) in query.iter() {
            if region_id == id {
                if *region_status == RegionStatus::Found {
                    atk_monster_with_player_skill.send(AtkMonsterWithPlayerSkill(*id));
                }
            }
        }
    }
}

pub fn visit_region(
    query: Query<(&RegionId, &RegionStatus), Without<EnemyMark>>,
    mut trigger_region_event: EventReader<RegionClickEvent>,
    mut change_region_status_event: EventWriter<ChangeRegionStatusEvent>,
) {
    for RegionClickEvent(id) in trigger_region_event.iter() {
        for (RegionId(region_id), region_status) in query.iter() {
            if region_id == id && *region_status == RegionStatus::Found {
                change_region_status_event
                    .send(ChangeRegionStatusEvent(*region_id, RegionStatus::Mist));
            }
        }
    }
}

pub fn visit_value_region(
    query: Query<(&RegionId, &RegionStatus, &Value)>,
    mut trigger_region_event: EventReader<RegionClickEvent>,
    mut player_status: ResMut<PlayerStatus>,
) {
    for RegionClickEvent(id) in trigger_region_event.iter() {
        for (RegionId(region_id), region_status, value) in query.iter() {
            if region_id == id && *region_status == RegionStatus::Found {
                for value in value.values.iter() {
                    match value {
                        crate::pool::values::KeyValue::PlayerCurrentHp(val) => {
                            player_status.cur_hp =
                                (player_status.cur_hp + val.to_i64()).min(player_status.max_hp);
                        }
                        crate::pool::values::KeyValue::PlayerAtk(val) => {
                            player_status.atk += val.to_i64()
                        }
                        crate::pool::values::KeyValue::PlayerDef(val) => {
                            player_status.def += val.to_i64()
                        }
                        crate::pool::values::KeyValue::PlayerMaxHp(val) => {
                            player_status.max_hp += val.to_i64()
                        }
                        crate::pool::values::KeyValue::PlayerGold(val) => {
                            player_status.gold += val.to_i64()
                        }
                    }
                }
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
                enemy.cur_hp = (enemy.cur_hp + val).max(0);
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
    mut visible_query: Query<(&mut Visibility, &RegionId)>,
    mut play_audio_event: EventWriter<PlayAudioEvent>,
) {
    let mut found_tiles = Vec::<&Tile>::new();
    for ev in change_region_status_event.iter() {
        let ChangeRegionStatusEvent(entity, ..) = ev;
        for (en, RegionId(region_id), status) in sprite_query.iter_mut() {
            match status {
                RegionStatus::Found => {
                    if region_id == entity {
                        commands.entity(en).insert(RegionStatus::Visited);
                        play_audio_event.send(PlayAudioEvent(AudioSound::Click));
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
            for (mut visibility, RegionId(region_id), ..) in visible_query.iter_mut() {
                if tile_id == *region_id {
                    visibility.is_visible = true;
                }
            }
        }
    }

    // hp_text_query
}

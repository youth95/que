use std::borrow::BorrowMut;
use std::collections::{HashMap, HashSet};
use std::iter::FromIterator;

use crate::components::TileType;
use crate::marks::{IDText, RegionId, RegionRect, RegionStatus};
use crate::pool::Pool;
use bevy::math::Vec3;
use bevy::prelude::Transform;

use super::pool::terrains::{AxisDirection, PlaneOrientation, Point};

#[derive(Debug, Clone)]
pub struct Tile {
    pub id: u64,
    pub adjacent: HashSet<u64>,
    pub shape: HashSet<Point>,
}

impl Tile {
    pub fn to_transform(&self, size: f32, gap: f32) -> Option<Transform> {
        if let Some(first_point) = self.shape.iter().next() {
            let Point(mut min_x, mut min_y) = first_point;
            let Point(mut max_x, mut max_y) = first_point;
            for pos in self.shape.clone().into_iter() {
                let Point(x, y) = pos;
                if x > max_x {
                    max_x = x;
                }
                if x < min_x {
                    min_x = x;
                }
                if y > max_y {
                    max_y = y;
                }
                if y < min_y {
                    min_y = y;
                }
            }
            let width = (max_x - min_x + 1) as f32;
            let height = (max_y - min_y + 1) as f32;
            let x = (min_x as f32 + (max_x - min_x) as f32 / 2.) as f32;
            let y = (min_y as f32 + (max_y - min_y) as f32 / 2.) as f32;
            return Some(Transform {
                translation: Vec3::new(x, y, 0.) * size,
                scale: Vec3::new(width, height, 0.) * size - gap,
                ..Default::default()
            });
        }
        None
    }
    pub fn to_tile_type(&self) -> TileType {
        if self.shape.len() == 1 {
            return TileType::Started;
        }
        if let Some(Point(first_point_x, first_point_y)) = self.shape.iter().next() {
            if self.shape.iter().all(|Point(x, ..)| x == first_point_x)
                || self.shape.iter().all(|Point(.., y)| y == first_point_y)
            {
                return TileType::Obstacle;
            }
        }
        TileType::Room
    }
}

#[derive(Debug)]
pub struct Regions {
    pub inc_id: u64,
    pub scope: (i64, i64),
    pub tile_map: HashMap<Point, u64>, // x,y,id
    pub tiles: HashMap<u64, Tile>,
}

impl Default for Regions {
    fn default() -> Self {
        Self {
            inc_id: 0,
            scope: (32, 32),
            tile_map: Default::default(),
            tiles: Default::default(),
        }
    }
}

impl Regions {
    pub fn make_tile(&mut self, pos: Point, steps: &[PlaneOrientation]) -> Tile {
        let shape = HashSet::from_iter(
            PlaneOrientation::to_points(steps)
                .into_iter()
                .map(|p| p + pos),
        );
        self.inc_id += 1;
        Tile {
            id: self.inc_id,
            adjacent: HashSet::default(),
            shape,
        }
    }

    /*
        按指定方向将tile调整到一个合适的位置并计算其连通性, 若未提供方向则每次都随机调整方向
    */
    pub fn adjust_tile(&mut self, tile: &mut Tile, axis_direction: Option<AxisDirection>) -> bool {
        // 若已在tile_map中存在则不会再次进行调整
        if self.tile_map.values().any(|k| *k == tile.id) {
            return false;
        }
        while tile.shape.iter().any(|p| self.tile_map.get(p) != None) {
            if let Some(axis_direction_value) = axis_direction {
                tile.shape =
                    HashSet::from_iter(tile.shape.iter().map(|p| *p + axis_direction_value));
            } else {
                let axis_direction_value_index =
                    (rand::random::<f32>() * AxisDirection::all().len() as f32).floor() as usize;
                let axis_direction_value = AxisDirection::all()[axis_direction_value_index];
                tile.shape =
                    HashSet::from_iter(tile.shape.iter().map(|p| *p + axis_direction_value));
            }
        }
        // 判断tile是否还在scope的限制当中
        if tile.shape.iter().all(|p| p.in_scope(self.scope)) {
            tile.shape.iter().for_each(|p| {
                // 标记边缘连通性
                for step in PlaneOrientation::all() {
                    let target_point = *p + PlaneOrientation::to_axis_direction(&step);
                    if let Some(id) = self.tile_map.get(&target_point) {
                        if *id != tile.id {
                            // 当前tile 添加联通标记
                            tile.adjacent.insert(*id);
                            // 被联通tile 添加联通标记
                            self.tiles.get_mut(id).unwrap().adjacent.insert(tile.id);
                        }
                    }
                }
                self.tile_map.insert(*p, tile.id);
            });
            // 将tile 加入tiles
            self.tiles.insert(tile.id, tile.clone());
            return true;
        }
        false
    }

    fn make_and_adjust_tile(
        &mut self,
        pos: Point,
        steps: &[PlaneOrientation],
        axis_direction: Option<AxisDirection>,
    ) -> Option<Tile> {
        let mut tile = self.make_tile(pos, steps);
        match self.adjust_tile(tile.borrow_mut(), axis_direction) {
            true => Some(tile),
            false => None,
        }
    }
    pub fn random_generate_tiles(
        &mut self,
        times: u64,
        move_steps_pool: &Pool<Vec<PlaneOrientation>>,
    ) {
        self.make_and_adjust_tile(Point(0, 0), &[], None); // 固定在中心生成一个1x1的方块
        for _ in 0..times {
            let x = ((rand::random::<f32>() - 0.5) as f64 * self.scope.0 as f64).floor() as i64;
            let y = ((rand::random::<f32>() - 0.5) as f64 * self.scope.1 as f64).floor() as i64;
            self.make_and_adjust_tile(Point(x, y), move_steps_pool.fetch_item().as_slice(), None);
        }
        // TODO 删除所有起始位置不连通的方块
    }
}

use crate::pool::terrains::get_plane_orientation_pool;
use bevy::prelude::*;

const SIZE: f32 = 32.;
const GAP: f32 = 4.;

const GEN_REGION_ITEMS: u64 = 32 * 32;

pub fn spawn_tiles_sprite_system(
    mut commands: Commands,
    mut regions: ResMut<Regions>,
    asset_server: Res<AssetServer>,
) {
    let font = asset_server.load("fonts/FiraSans-Bold.ttf");
    let text_style = TextStyle {
        font,
        font_size: 20.0,
        color: Color::WHITE,
    };
    let text_alignment = TextAlignment {
        vertical: VerticalAlign::Center,
        horizontal: HorizontalAlign::Center,
    };
    let pool: Pool<Vec<PlaneOrientation>> = get_plane_orientation_pool();
    regions.random_generate_tiles(GEN_REGION_ITEMS, &pool);
    for (_, tile) in regions.tiles.iter() {
        let transform = tile.to_transform(SIZE, GAP).unwrap();
        let region_id = RegionId(tile.id);
        let tile_type = tile.to_tile_type();
        let region_status: RegionStatus = match tile_type {
            TileType::Started => RegionStatus::Found,
            _ => RegionStatus::Mist,
        };
        commands
            .spawn()
            .insert_bundle(SpriteBundle::default())
            .insert(transform)
            .insert(Sprite {
                color: Color::BLUE,
                ..Default::default()
            })
            .insert(region_status)
            .insert(RegionRect)
            .insert(region_id);
        commands
            .spawn_bundle(Text2dBundle {
                text: Text::with_section(
                    format!("{}", tile.id),
                    text_style.clone(),
                    text_alignment,
                ),
                visibility: Visibility { is_visible: false },
                transform: Transform {
                    translation: Vec3::new(transform.translation.x, transform.translation.y, 1.),
                    ..Default::default()
                },
                ..Default::default()
            })
            .insert(IDText)
            .insert(region_id.clone());
    }
}

pub struct TriggerRegionEvent(pub u64);

pub fn trigger_region_system(
    mut commands: Commands,
    mut trigger_region_event: EventReader<TriggerRegionEvent>,
    regions: ResMut<Regions>,
    mut sprite_query: Query<(Entity, &RegionId, &RegionStatus), With<RegionRect>>,
) {
    let mut found_tiles = Vec::<&Tile>::new();
    for ev in trigger_region_event.iter() {
        let TriggerRegionEvent(entity) = ev;
        for (en, RegionId(region_id), status) in sprite_query.iter_mut() {
            match status {
                RegionStatus::Found => {
                    if region_id == entity {
                        commands.entity(en).insert(RegionStatus::Visited);
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
                            commands.entity(en).insert(RegionStatus::Found);
                        }
                    }
                    _ => (),
                }
            }
        }
    }
}

pub fn region_rect_color_system(
    mut region_react_query: Query<(&mut Sprite, &RegionStatus), With<RegionRect>>,
) {
    for (mut sprite, status) in region_react_query.iter_mut() {
        match status {
            RegionStatus::Mist => {
                sprite.color = Color::BLACK;
            }
            RegionStatus::Found => {
                sprite.color = Color::GREEN;
            }
            RegionStatus::Visited => {
                sprite.color = Color::GRAY;
            }
        }
    }
}

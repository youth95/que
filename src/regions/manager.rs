use std::borrow::BorrowMut;
use std::collections::HashMap;
use std::iter::FromIterator;

use crate::components::TileType;
use crate::pool::Pool;
use bevy::math::Vec3;
use bevy::prelude::Transform;
use bevy::utils::HashSet;

use super::super::pool::terrains::{AxisDirection, PlaneOrientation, Point};

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

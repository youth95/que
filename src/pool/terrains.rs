use serde::{Deserialize, Serialize};

use super::Pool;
struct DirectionPoint(Direction, Direction);

#[derive(Clone, Copy)]
pub struct AxisDirection(pub Axis, pub Direction);

impl AxisDirection {
    pub fn all() -> Vec<AxisDirection> {
        vec![
            AxisDirection(Axis::X, Direction::Just),
            AxisDirection(Axis::Y, Direction::Just),
            AxisDirection(Axis::Y, Direction::Burden),
            AxisDirection(Axis::Y, Direction::Burden),
        ]
    }
}

#[derive(Debug, Clone, Copy, Hash, std::cmp::Eq)]
pub struct Point(pub i64, pub i64);

impl std::cmp::PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Point {
    pub fn in_scope(&self, scope: (i64, i64)) -> bool {
        let Point(x, y) = self;
        x.abs() < scope.0 && y.abs() < scope.1
    }
}

impl std::ops::Add<Point> for Point {
    type Output = Point;

    fn add(self, rhs: Point) -> Self::Output {
        Point(self.0 + rhs.0, self.1 + rhs.1)
    }
}

impl std::ops::Add<AxisDirection> for Point {
    type Output = Point;

    fn add(self, rhs: AxisDirection) -> Self::Output {
        let Point(x, y) = self;
        let AxisDirection(axis, direction) = rhs;
        match axis {
            Axis::X => match direction {
                Direction::Just => Point(x + 1, y),
                Direction::Burden => Point(x - 1, y),
            },
            Axis::Y => match direction {
                Direction::Just => Point(x, y + 1),
                Direction::Burden => Point(x, y - 1),
            },
        }
    }
}

impl std::ops::Add<DirectionPoint> for Point {
    type Output = Point;
    fn add(self, rhs: DirectionPoint) -> Self::Output {
        let DirectionPoint(dx, dy) = rhs;
        self + AxisDirection(Axis::X, dx) + AxisDirection(Axis::Y, dy)
    }
}

#[derive(Clone, Copy)]
pub enum Direction {
    /* 正方向 */
    Just,
    /* 负方向 */
    Burden,
}

#[derive(Clone, Copy)]
pub enum Axis {
    X,
    Y,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum PlaneOrientation {
    Up,
    Down,
    Left,
    Right,
}

impl PlaneOrientation {
    pub fn all() -> Vec<PlaneOrientation> {
        vec![
            PlaneOrientation::Up,
            PlaneOrientation::Down,
            PlaneOrientation::Left,
            PlaneOrientation::Right,
        ]
    }

    pub fn to_axis_direction(plane_orientation: &PlaneOrientation) -> AxisDirection {
        match plane_orientation {
            PlaneOrientation::Up => AxisDirection(Axis::Y, Direction::Just),
            PlaneOrientation::Down => AxisDirection(Axis::Y, Direction::Burden),
            PlaneOrientation::Right => AxisDirection(Axis::X, Direction::Just),
            PlaneOrientation::Left => AxisDirection(Axis::X, Direction::Burden),
        }
    }

    pub fn to_points(steps: &[PlaneOrientation]) -> Vec<Point> {
        let mut new_pos = Point(0, 0);
        let mut result = vec![new_pos];
        for step in steps {
            new_pos = new_pos + PlaneOrientation::to_axis_direction(step);
            result.push(new_pos);
        }
        result
    }
}

pub fn get_plane_orientation_pool() -> Pool<Vec<PlaneOrientation>> {
    let config = include_str!("../../assets/pool/terrains.ron");
    ron::from_str(config).unwrap()
}
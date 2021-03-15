use crate::prelude::*;

pub struct ForageMap {
    pub tiles: Vec<TileType>,
    pub width: i32,
    pub height: i32,
    pub nest_positions: Vec<usize>,
    pub forage_positions: Vec<usize>,
}

impl ForageMap {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            tiles: Vec::<TileType>::new(),
            width,
            height,
            nest_positions: Vec::<usize>::new(),
            forage_positions: Vec::<usize>::new(),
        }
    }

    pub fn update_tiles(&mut self, updated_tiles: Vec<TileType>) {
        self.tiles = updated_tiles;
    }

    pub fn try_idx(&self, point: Point) -> Option<usize> {
        if self.in_bounds(point) {
            Some(self.point2d_to_index(point))
        } else {
            None
        }
    }

    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point)
            && (self.tiles[self.point2d_to_index(point)] == TileType::Floor
                || self.tiles[self.point2d_to_index(point)] == TileType::FloorVar1
                || self.tiles[self.point2d_to_index(point)] == TileType::FloorVar2
                || self.tiles[self.point2d_to_index(point)] == TileType::FloorVar3
                || self.tiles[self.point2d_to_index(point)] == TileType::Exit)
    }

    fn valid_exit(&self, loc: Point, delta: Point) -> Option<usize> {
        let destination = loc + delta;
        if self.in_bounds(destination) {
            let idx = self.point2d_to_index(destination);
            Some(idx)
        } else {
            None
        }
    }
}

impl Algorithm2D for ForageMap {
    fn dimensions(&self) -> Point {
        Point::new(SCREEN_WIDTH, SCREEN_HEIGHT)
    }
}

impl BaseMap for ForageMap {
    fn is_opaque(&self, idx: usize) -> bool {
        self.tiles[idx as usize] != TileType::Floor
            && self.tiles[idx as usize] != TileType::FloorVar1
            && self.tiles[idx as usize] != TileType::FloorVar2
            && self.tiles[idx as usize] != TileType::FloorVar3
    }

    fn get_available_exits(&self, idx: usize) -> SmallVec<[(usize, f32); 10]> {
        let mut exits = SmallVec::new();
        let location = self.index_to_point2d(idx);

        if let Some(idx) = self.valid_exit(location, Point::new(-1, 0)) {
            let point = location + Point::new(-1, 0);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 0)) {
            let point = location + Point::new(1, 0);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, -1)) {
            let point = location + Point::new(0, -1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(0, 1)) {
            let point = location + Point::new(0, 1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, -1)) {
            let point = location + Point::new(-1, -1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, -1)) {
            let point = location + Point::new(1, -1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(-1, 1)) {
            let point = location + Point::new(-1, 1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }
        if let Some(idx) = self.valid_exit(location, Point::new(1, 1)) {
            let point = location + Point::new(1, 1);
            if self.can_enter_tile(point) {
                exits.push((idx, 1.0))
            } else {
                exits.push((idx, 2.0))
            }
        }

        exits
    }

    fn get_pathing_distance(&self, idx1: usize, idx2: usize) -> f32 {
        DistanceAlg::Diagonal.distance2d(self.index_to_point2d(idx1), self.index_to_point2d(idx2))
    }
}

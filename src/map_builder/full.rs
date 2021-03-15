use super::MapArchitect;
use crate::prelude::*;

pub struct EmptyForagingArchitect {}

impl MapArchitect for EmptyForagingArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder {
        let mut mb = MapBuilder {
            map: Map::new(SCREEN_WIDTH, SCREEN_HEIGHT),
            rooms: Vec::new(),
            monster_spawns: Vec::new(),
            theme: super::themes::RootedTheme::new(),
        };
        mb.fill(TileType::Floor);

        for i in 0..2 {
            let center_idx = rng.random_slice_index(&mb.map.tiles).unwrap();
            let center_pt = mb.map.index_to_point2d(center_idx);
            if i == 0 {
                mb.map.forage_map.nest_positions.push(center_idx)
            } else {
                mb.map.forage_map.forage_positions.push(center_idx);
            }
            let r = rng.range(6, 12);
            for circle_point in BresenhamCircleNoDiag::new(center_pt, r) {
                let line = Bresenham::new(center_pt, circle_point);
                for line_point in line {
                    if mb.map.in_bounds(line_point) {
                        let pt_idx = mb.map.point2d_to_index(line_point);
                        mb.map.tiles[pt_idx] = TileType::FloorVar1;
                    }
                }
            }
        }
        mb
    }
}

use crate::prelude::*;
mod automata;
use automata::CellularAutomataArchitect;
mod drunkard;
use drunkard::DrunkardsWalkArchitect;
mod themes;
pub use themes::*;

const UNREACHABLE: &f32 = &f32::MAX;

trait MapArchitect {
    fn new(&mut self, rng: &mut RandomNumberGenerator) -> MapBuilder;
}

pub trait MapTheme: Sync + Send {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType;
    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair;
    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair;
}

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>,
    pub monster_spawns: Vec<Point>,
    pub player_start: Point,
    pub amulet_start: Point,
    pub theme: Box<dyn MapTheme>,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut architect: Box<dyn MapArchitect> = match rng.range(0, 4) {
            0 | 1 | 2 => Box::new(CellularAutomataArchitect {}),
            _ => Box::new(DrunkardsWalkArchitect {}),
        };
        let mut mb = architect.new(rng);
        mb.theme = RootedTheme::new();
        mb.tile_variety(rng);
        mb
    }

    fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }

    fn find_most_distant(&self) -> Point {
        let dijkstra_map = DijkstraMap::new(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            &[self.map.point2d_to_index(self.player_start)],
            &self.map,
            1024.0,
        );

        self.map.index_to_point2d(
            dijkstra_map
                .map
                .iter()
                .enumerate()
                .filter(|(_, dist)| *dist < UNREACHABLE)
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap())
                .unwrap()
                .0,
        )
    }
    fn spawn_monsters(&self, start: Point, rng: &mut RandomNumberGenerator) -> Vec<Point> {
        const NUM_ENTITIES: usize = 100;
        let mut spawnable_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(idx, t)| {
                **t == TileType::Floor
                    && DistanceAlg::Pythagoras.distance2d(start, self.map.index_to_point2d(*idx))
                        > 10.0
            })
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let mut spawns = Vec::new();
        for _ in 0..NUM_ENTITIES {
            let target_index = rng.random_slice_index(&spawnable_tiles).unwrap();
            spawns.push(spawnable_tiles[target_index]);
            spawnable_tiles.remove(target_index);
        }
        spawns
    }

    fn tile_variety(&mut self, rng: &mut RandomNumberGenerator) {
        let floor_tiletypes = [
            TileType::FloorVar1,
            TileType::FloorVar2,
            TileType::FloorVar3,
        ];
        let mut floor_tiles: Vec<Point> = self
            .map
            .tiles
            .iter()
            .enumerate()
            .filter(|(_, t)| **t == TileType::Floor)
            .map(|(idx, _)| self.map.index_to_point2d(idx))
            .collect();

        let var_tile_amount = floor_tiles.len() / 10;

        for _ in 0..var_tile_amount {
            let idx = rng.random_slice_index(&floor_tiles).unwrap();
            let point = floor_tiles[idx];
            let point_idx = self.map.point2d_to_index(point);
            self.map.tiles[point_idx] = *rng.random_slice_entry(&floor_tiletypes).unwrap();
            floor_tiles.remove(idx);
        }
    }
}

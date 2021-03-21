use crate::prelude::*;

pub struct RootedTheme {}

impl RootedTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for RootedTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::FloorVar1 => to_cp437(':'), //rubble
            TileType::FloorVar2 => to_cp437(','), //smalfoilage
            TileType::FloorVar3 => to_cp437(';'), //bigfoilage
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }

    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            TileType::Floor => ColorPair::new(
                RGB::from_hex("#86949F").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::FloorVar1 => ColorPair::new(
                RGB::from_hex("#716E61").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::FloorVar2 => ColorPair::new(
                RGB::from_hex("#71AA34").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::FloorVar3 => ColorPair::new(
                RGB::from_hex("#215E21").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),

            TileType::Wall => ColorPair::new(
                RGB::from_hex("#4e4a4e").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            TileType::Exit => ColorPair::new(
                RGB::from_hex("#D4A798").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
        }
    }

    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            TileType::Exit => ColorPair::new(
                RGB::from_hex("#D4A798").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
            _ => ColorPair::new(
                RGB::from_hex("#333366").unwrap(),
                RGB::from_hex("#17111D").unwrap(),
            ),
        }
    }
}

pub struct TiledTheme {}

impl TiledTheme {
    pub fn new() -> Box<dyn MapTheme> {
        Box::new(Self {})
    }
}

impl MapTheme for TiledTheme {
    fn tile_to_render(&self, tile_type: TileType) -> FontCharType {
        match tile_type {
            TileType::Floor => to_cp437('.'),
            TileType::FloorVar1 => to_cp437(':'), //rubble
            TileType::FloorVar2 => to_cp437(','), //smalfoilage
            TileType::FloorVar3 => to_cp437(';'), //bigfoilage
            TileType::Wall => to_cp437('#'),
            TileType::Exit => to_cp437('>'),
        }
    }

    fn in_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            _ => ColorPair::new(WHITE, BLACK),
        }
    }

    fn out_fov_colorpair_to_render(&self, tile_type: TileType) -> ColorPair {
        match tile_type {
            _ => ColorPair::new(DARK_GRAY, BLACK),
        }
    }
}

use f2_common_format::reader::ToDo;
use fo_net_protocol::generics::{collections::RepeatExt, length::LenConst};
use represent_derive::{MakeWith, Visit};

use crate::slots::Levels;

#[derive(Debug, MakeWith, Visit)]
pub struct Tiles {
    levels: Levels<LevelTiles>,
}

const TILE_GRID_WIDTH: usize = 100;
const TILE_GRID_HEIGHT: usize = 100;
const TILE_GRID_AREA: usize = TILE_GRID_WIDTH * TILE_GRID_HEIGHT;

#[derive(Debug, MakeWith, Visit)]
pub struct LevelTiles {
    grid: RepeatExt<TilePair, LenConst<TILE_GRID_AREA>>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct TilePair {
    roof: TileId,
    floor: TileId,
}

#[derive(Debug, MakeWith, Visit)]
struct TileId(ToDo<u16>);
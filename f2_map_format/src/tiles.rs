use f2_common_format::reader::ToDo;
use represent::{MakeWith, VisitWith};
use represent_extra::generics::{collections::RepeatExt, length::LenConst};

use crate::slots::Levels;

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Tiles {
    levels: Levels<LevelTiles>,
}

const TILE_GRID_WIDTH: usize = 100;
const TILE_GRID_HEIGHT: usize = 100;
const TILE_GRID_AREA: usize = TILE_GRID_WIDTH * TILE_GRID_HEIGHT;

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct LevelTiles {
    grid: RepeatExt<TilePair, LenConst<TILE_GRID_AREA>>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct TilePair {
    roof: TileId,
    floor: TileId,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TileId(ToDo<u16>);

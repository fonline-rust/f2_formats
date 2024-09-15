use f2_common_format::{
    reader::{ToDo, SLOT_SUB_TYPE},
    MiscSubType,
};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::Load;

#[derive(Debug, MakeWith, VisitWith)]
#[alt(ty = "Load<MiscSubType, SLOT_SUB_TYPE>")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Misc {
    #[alt("Load(MiscSubType::Generic)")]
    Generic,
    #[alt("Load(MiscSubType::ExitGrid)")]
    ExitGrid(ExitGrid),
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ExitGrid {
    /// EXIT-MAP-ID">EXIT-MAP-ID : Map Id. The id of the map that this exit grid leads to.
    /// Fallout 1: Map filename found in map.msg
    /// Fallout 2: Map details found in data/maps.txt in section [Map id]
    map_id: ToDo<u32>,
    /// Player position. Position on the hex grid that the player will start in when moving to map EXIT-MAP-ID.
    hex: ToDo<u32>,
    /// Map elevation. Elevation of map EXIT-MAP-ID that this exit grid leads to.
    elevation: ToDo<u32>,
    /// Player orientation. Orientation of the player when entering EXIT-MAP-ID from this exit grid.
    dir: ToDo<u32>,
}

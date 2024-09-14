use fo_net_protocol::generics::slots::Load;
use represent_derive::{MakeWith, Visit};
use f2_common_format::{reader::{SLOT_SUB_TYPE, ToDo}, MiscSubType};

#[derive(Debug, MakeWith, Visit)]
#[alt(ty = "Load<MiscSubType, SLOT_SUB_TYPE>")]
pub enum Misc {
    #[alt("Load(MiscSubType::Generic)")]
    Generic,
    #[alt("Load(MiscSubType::ExitGrid)")]
    ExitGrid(ExitGrid),
}

#[derive(Debug, MakeWith, Visit)]
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

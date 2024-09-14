use fo_net_protocol::generics::slots::Load;
use represent_derive::{MakeWith, Visit};
use f2_common_format::{reader::{SLOT_SUB_TYPE, ToDo}, ScenerySubType, Destination};

#[derive(Debug, MakeWith, Visit)]
#[alt(ty = "Load<ScenerySubType, SLOT_SUB_TYPE>")]
pub enum Scenery {
    #[alt("Load(ScenerySubType::Door)")]
    Door(Door),
    #[alt("Load(ScenerySubType::Stairs)")]
    Stairs(Stairs),
    #[alt("Load(ScenerySubType::Elevator)")]
    Elevator(Elevator),
    #[alt("Load(ScenerySubType::LadderBottom)")]
    LadderBottom(Ladder),
    #[alt("Load(ScenerySubType::LadderTop)")]
    LadderTop(Ladder),
    #[alt("Load(ScenerySubType::Generic)")]
    Generic,
}

#[derive(Debug, MakeWith, Visit)]
pub struct Door {
    walk_through: ToDo<u32>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct Elevator {
    /// Elevator type
    ty: ToDo<u32>,
    /// Current level of the elevator (not to be confused with the level of map!).
    /// This parameter specifies which floor the arrow initially points at. 
    level: ToDo<u32>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct Stairs {
    destination: Destination,
    /// Destination map
    map: ToDo<u32>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct Ladder {
    destination: Destination,
    /// Destination map
    map: ToDo<u32>,
}

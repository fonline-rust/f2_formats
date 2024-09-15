use f2_common_format::{
    reader::{ToDo, SLOT_SUB_TYPE},
    Destination, ScenerySubType,
};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::Load;

#[derive(Debug, MakeWith, VisitWith)]
#[alt(ty = "Load<ScenerySubType, SLOT_SUB_TYPE>")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
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

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Door {
    walk_through: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Elevator {
    /// Elevator type
    ty: ToDo<u32>,
    /// Current level of the elevator (not to be confused with the level of map!).
    /// This parameter specifies which floor the arrow initially points at.
    level: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Stairs {
    destination: Destination,
    /// Destination map
    map: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ladder {
    destination: Destination,
    /// Destination map
    map: ToDo<u32>,
}

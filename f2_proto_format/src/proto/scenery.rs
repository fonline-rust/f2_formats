use f2_common_format::{reader::SLOT_SUB_TYPE, Destination, ScenerySubType};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::{Load, Store};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Scenery {
    wall_light_flags: ToDo<u16>,
    action_flags: ToDo<u16>,
    script_id: ToDo<u32>,
    ty: Store<ScenerySubType, SLOT_SUB_TYPE>,
    material_id: ToDo<u32>,
    sound_id: ToDo<u8>,
    kind: SceneryKind,
}

impl Scenery {
    pub fn sub_type(&self) -> ScenerySubType {
        self.ty.inner
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[alt(ty = "Load<ScenerySubType, SLOT_SUB_TYPE>")]
enum SceneryKind {
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
    Generic(GenericScenery),
}

#[derive(Debug, MakeWith, VisitWith)]
struct Door {
    /// Values:
    /// 0x0000000F: yes
    walk_through: ToDo<u32>,
    /// 0xCCCCCCCC (mostly) or 0xFFFFFFFF (sometimes)
    unknown: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
struct Stairs {
    destination: Destination,
    /// Destination map
    /// Value is a map number in data\maps.txt (hard-coded list in Fallout?), -1 goes to the worldmap.
    map: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
struct Elevator {
    /// Elevator type
    /// Values from 0x00 to 0x17
    ty: ToDo<u32>,
    /// Current level of the elevator (not to be confused with the level of map!).
    /// This parameter specifies which floor the arrow initially points at.
    level: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
struct Ladder {
    destination: Destination,
}

#[derive(Debug, MakeWith, VisitWith)]
struct GenericScenery {
    /// 0xCCCCCCCC (mostly) or 0xFFFFFFFF (sometimes)
    unknown: ToDo<u32>,
}

use f2_common_format::{
    reader::{F2ReaderError, ToDo, SLOT_OBJECT_TYPE},
    Fid, ObjectPid, ObjectType, ProtoInfo,
};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::Load;

use self::{critter::Critter, item::Item, misc::Misc, scenery::Scenery, tile::Tile, wall::Wall};

mod critter;
mod item;
mod misc;
mod scenery;
mod tile;
mod wall;

#[derive(Debug, MakeWith)]
pub struct Proto {
    common: ProtoCommon,
    kind: ProtoKind,
}

impl Proto {
    pub fn common(&self) -> &ProtoCommon {
        &self.common
    }

    pub fn kind(&self) -> &ProtoKind {
        &self.kind
    }
}

impl ProtoInfo for Proto {
    fn sub_type(&self) -> Option<u32> {
        Some(match &self.kind {
            ProtoKind::Item(item) => item.sub_type() as _,
            ProtoKind::Scenery(scenery) => scenery.sub_type() as _,
            ProtoKind::Misc(misc) => misc.sub_type(&self.common.proto_pid) as _,
            _ => return None,
        })
    }
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct ProtoCommon {
    pub proto_pid: ObjectPid,
    text_id: ToDo<u32>,
    pub frm_id: Fid,
    light_radius: ToDo<u32>,
    light_intensity: ToDo<u32>,
    flags: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[alt(
    ty = "Load<ObjectType, SLOT_OBJECT_TYPE>",
    err = "F2ReaderError",
    default = "Err(F2ReaderError::InvalidObjectType.into())"
)]
pub enum ProtoKind {
    #[alt("Load(ObjectType::Item)")]
    Item(Item),
    #[alt("Load(ObjectType::Critter)")]
    Critter(Critter),
    #[alt("Load(ObjectType::Scenery)")]
    Scenery(Scenery),
    #[alt("Load(ObjectType::Wall)")]
    Wall(Wall),
    #[alt("Load(ObjectType::Tile)")]
    Tile(Tile),
    #[alt("Load(ObjectType::Misc)")]
    Misc(Misc),
}

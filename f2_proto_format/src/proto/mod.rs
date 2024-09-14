use self::{item::Item, critter::Critter, scenery::Scenery, wall::Wall, tile::Tile, misc::Misc};
use f2_common_format::{ObjectType, Pid, ObjectPid, Fid, reader::{SLOT_OBJECT_TYPE, ToDo, F2ReaderError}, ProtoInfo};
use represent_derive::{MakeWith, Visit};
use fo_net_protocol::generics::slots::Load;

mod item;
mod critter;
mod scenery;
mod wall;
mod tile;
mod misc;

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

#[derive(Debug, MakeWith, Visit)]
pub struct ProtoCommon {
    pub proto_pid: ObjectPid,
    text_id: ToDo<u32>,
    pub frm_id: Fid,
    light_radius: ToDo<u32>,
    light_intensity: ToDo<u32>,
    flags: ToDo<u32>,
}

#[derive(Debug, MakeWith, Visit)]
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

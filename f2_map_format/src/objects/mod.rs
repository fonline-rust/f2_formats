use f2_common_format::{
    reader::{
        F2Context, F2ReaderError, Pod, SlotsSpace, ToDo, SLOT_INVENTORY_COUNT, SLOT_OBJECT_TYPE,
        SLOT_SUB_TYPE,
    },
    Fid, GetProto, ObjectPid, ObjectType, Pid, ProtoInfo,
};
use represent::{MakeType, MakeWith, Maker, VisitWith};
use represent_extra::{
    generics::{
        slots::{Load, Slots, Store},
        Has, HasValue,
    },
    typedefs::{RepeatMake, RepeatSlot},
};

use self::{item::Item, misc::Misc, scenery::Scenery};
use crate::{slots::Levels, Hex, Unknown, Unused};

mod item;
mod misc;
mod scenery;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Objects {
    _total_objects_count: Pod<u32>,
    levels: Levels<LevelObjects>,
}

impl Objects {
    pub fn all_objects<'a>(&'a self) -> [impl 'a + ExactSizeIterator<Item = &'a LevelObject>; 3] {
        let map = move |level: Option<&'a LevelObjects>| {
            level
                .map(|level| level.objects.0.as_slice())
                .unwrap_or(&[])
                .iter()
                .map(move |obj| &obj.0)
        };
        [
            map(self.levels.level_0.0.as_ref()),
            map(self.levels.level_1.0.as_ref()),
            map(self.levels.level_2.0.as_ref()),
        ]
    }
}

#[derive(Debug, MakeWith, VisitWith)]
struct LevelObjects {
    objects: RepeatMake<Pod<u32>, SlotsSpace<LevelObject>>,
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct LevelObject {
    pub object: Object,
    pub inventory: ObjectInventory,
}

#[derive(Debug)]
pub struct Object {
    pub common: ObjectCommon,
    pub kind: ObjectKind,
}

impl<M: Maker + MakeType<ObjectCommon> + MakeType<ObjectKind> + Has<Slots> + HasValue<F2Context>>
    MakeWith<M> for Object
where
    <M as HasValue<F2Context>>::Value: GetProto,
    <M as Maker>::Error: From<F2ReaderError>,
{
    fn make_with(maker: &mut M) -> Result<Self, M::Error> {
        let common: ObjectCommon = maker.make()?;
        let pid = common.proto_id.pid();
        let proto = maker
            .give_value()
            .get_proto(pid)
            .ok_or(F2ReaderError::ProtoNotFound(pid))?;
        if let Some(sub_type) = proto.sub_type() {
            let slots = maker.give_mut();
            slots
                .store(SLOT_SUB_TYPE, sub_type)
                .expect("Store sub type only once");
        }
        let kind = maker.make()?;
        Ok(Self { common, kind })
    }
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct ObjectCommon {
    separator: Unused<u32>,
    hex: Hex,
    x: ToDo<u32>,
    y: ToDo<u32>,
    sx: ToDo<i32>,
    sy: ToDo<i32>,
    frame_number: ToDo<u32>,
    orientation: ToDo<u32>,
    frm_id: Fid,
    unknown_flags: Unknown<u32>,
    map_elevation: ToDo<u32>,
    proto_id: ObjectPid,
    critter_index_number: ToDo<i32>,
    light_radius: ToDo<u32>,
    light_intensity: ToDo<u32>,
    outline_color: ToDo<u32>,
    map_scripts_id: Pid,
    script_id: ToDo<i32>,
    inventory_count: Store<Pod<u32>, SLOT_INVENTORY_COUNT>,
    inventory_size: ToDo<u32>,
    unknown_10: Unknown<u32>,
    unknown_11: Unknown<u32>,
}

impl ObjectCommon {
    pub fn inventory_count(&self) -> u32 {
        self.inventory_count.inner.0
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[alt(
    ty = "Load<ObjectType, SLOT_OBJECT_TYPE>",
    err = "F2ReaderError",
    default = "Err(F2ReaderError::InvalidObjectType.into())"
)]
pub enum ObjectKind {
    #[alt("Load(ObjectType::Item)")]
    Item(Item),
    #[alt("Load(ObjectType::Critter)")]
    Critter(Critter),
    #[alt("Load(ObjectType::Scenery)")]
    Scenery(Scenery),
    #[alt("Load(ObjectType::Wall)")]
    Wall,
    #[alt("Load(ObjectType::Tile)")]
    Tile,
    #[alt("Load(ObjectType::Misc)")]
    Misc(Misc),
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct ObjectInventory {
    slots: RepeatSlot<SlotsSpace<InventoryObject>, SLOT_INVENTORY_COUNT>,
}

impl ObjectInventory {
    pub fn slots(&self) -> impl ExactSizeIterator<Item = &InventoryObject> {
        self.slots.0.iter().map(|slot| &slot.0)
    }
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct InventoryObject {
    quantity: Pod<u32>,
    object: Object,
}
impl InventoryObject {
    pub fn quantity(&self) -> u32 {
        self.quantity.0
    }

    pub fn object(&self) -> &Object {
        &self.object
    }
}

#[derive(Debug, MakeWith, VisitWith)]
pub struct Critter {
    unknown: Unknown<[u32; 7]>,
    /// Current Hit Points
    health_points: ToDo<i32>,
    /// Current Radidation
    radiation: ToDo<u32>,
    /// Current Poisoning
    poison: ToDo<u32>,
}

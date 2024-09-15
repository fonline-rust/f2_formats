use reader::{F2Reader, ToDo, SLOT_SCRIPT_TYPE};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::Store;

use self::reader::{Pod, SLOT_OBJECT_TYPE};
use crate::reader::F2ReaderError;

mod fid;
pub mod reader;
mod sniffer;

pub use self::fid::{Fid, FrmType};

#[derive(Debug, MakeWith, VisitWith, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pid {
    ty: ObjectType,
    dummy: Pod<u8>,
    id: Pod<u16>,
}

impl Pid {
    pub fn ty(&self) -> ObjectType {
        self.ty
    }

    pub fn id(&self) -> u16 {
        self.id.0
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ObjectPid {
    ty: Store<ObjectType, SLOT_OBJECT_TYPE>,
    dummy: Pod<u8>,
    id: Pod<u16>,
}

impl ObjectPid {
    pub fn ty(&self) -> ObjectType {
        self.ty.inner
    }

    pub fn pid(&self) -> Pid {
        let Self {
            ty: Store { inner: ty },
            dummy,
            id,
        } = *self;
        Pid { ty, dummy, id }
    }

    pub fn id(&self) -> u16 {
        self.id.0
    }
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[repr(u8)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ObjectType {
    Item = 0,
    Critter = 1,
    Scenery = 2,
    Wall = 3,
    Tile = 4,
    Misc = 5,
    //Interface = 6,
    //Inventory = 7,
    //Heads = 8,
    //Background = 9,
    Invalid = 255,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Sid {
    ty: Store<ScriptType, SLOT_SCRIPT_TYPE>,
    dummy: Pod<u8>,
    id: Pod<u16>,
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u8)]
pub enum ScriptType {
    System = 0,
    Spatial = 1,
    Timer = 2,
    Item = 3,
    Critter = 4,
    Invalid,
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ItemSubType {
    Armor = 0,
    Container = 1,
    Drug = 2,
    Weapon = 3,
    Ammo = 4,
    Misc = 5,
    Key = 6,
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum ScenerySubType {
    Door = 0,
    Stairs = 1,
    Elevator = 2,
    LadderBottom = 3,
    LadderTop = 4,
    Generic = 5,
}

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy)]
#[repr(u32)]
pub enum MiscSubType {
    Generic = 0,
    ExitGrid = 1,
}

/// Values:
/// (see perk.msg, starting with the line 101)
/// -1 for no perk
#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Perk(Pod<i32>);

#[derive(Debug, num_enum::TryFromPrimitive, Clone, Copy)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
#[repr(u32)]
pub enum MapVersion {
    //Fallout1 = 19,
    Fallout2 = 20,
}

impl<'a, C> represent::MakeType<ScriptType> for F2Reader<'a, C> {
    fn make_type(&mut self) -> Result<ScriptType, F2ReaderError> {
        use num_enum::TryFromPrimitive;
        let pod: Pod<<ScriptType as TryFromPrimitive>::Primitive> = self.make_type()?;
        Ok(ScriptType::try_from_primitive(pod.0).unwrap_or(ScriptType::Invalid))
    }
}
impl From<ScriptType> for u32 {
    fn from(value: ScriptType) -> Self {
        value as _
    }
}

reader::make_primitive_enum!(ObjectType, ItemSubType, ScenerySubType, FrmType, MapVersion);

macro_rules! try_from_try_into {
    ($from:ty => $( $into:ty ),+ $(,)?) => {
        $(
            impl TryFrom<$from> for $into {
                type Error = F2ReaderError;

                fn try_from(value: $from) -> Result<Self, Self::Error> {
                    use num_enum::TryFromPrimitive;
                    Self::try_from_primitive(value.try_into().map_err(F2ReaderError::try_from_primitive)?).map_err(F2ReaderError::try_from_primitive)
                }
            }
        )+
    };
}

try_from_try_into!(u32 => ObjectType, ScriptType, FrmType);

pub trait ProtoInfo {
    fn sub_type(&self) -> Option<u32>;
}

pub trait GetProto {
    type Proto: ProtoInfo;
    fn get_proto(&self, proto_id: Pid) -> Option<&Self::Proto>;
}

/// DestTile & DestElev
/// Format:[1]
///
/// 0xABCCDDDD
///   | | |
///   | | |
///   | | +- DestTile
///   | +--- Possible part of DestElev
///   +----- DestElev
///
/// DestElev:
/// A can be anything ( setting DestElev in Fallout 2 mapper autmatically sets A to 0xC )
/// B values:
///   0x0: zero
///   0x2: first
///   0x4: second
///
/// DestTile values: 0 to 40000.
#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Destination {
    todo: ToDo<u32>,
}

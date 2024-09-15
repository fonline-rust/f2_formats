use std::path::Path;

use f2_common_format::{
    reader::{self, ToDo},
    GetProto,
};
use represent::{MakeType, MakeWith, VisitWith};
use represent_extra::typedefs::TailBytes;

mod header;
mod objects;
mod scripts;
mod tiles;
mod variables;

pub use self::objects::LevelObject;
use self::{
    header::Header, objects::Objects, scripts::Scripts, tiles::Tiles, variables::Variables,
};

mod slots {
    use represent::{MakeWith, VisitWith};
    use represent_extra::generics::condition::{Conditional, Flag, Not};

    pub const NUM_LOCAL_VARS: usize = 0;
    pub const MAP_FLAGS: usize = 1;
    pub const NUM_GLOBAL_VARS: usize = 2;

    pub type LevelCondition<T, const FLAG: u32> = Conditional<Not<Flag<FLAG, MAP_FLAGS>>, T>;

    #[derive(Debug, MakeWith, VisitWith)]
    #[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
    pub struct Levels<T> {
        pub level_0: LevelCondition<T, 0x2>,
        pub level_1: LevelCondition<T, 0x4>,
        pub level_2: LevelCondition<T, 0x8>,
    }
}

#[derive(Debug, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Map {
    header: Header,
    variables: Variables,
    tiles: Tiles,
    scripts: Scripts,
    objects: Objects,
    tail: TailBytes,
}

impl<
    M: represent::Maker
        + MakeType<Header>
        + MakeType<Variables>
        + MakeType<Tiles>
        + MakeType<Scripts>
        + MakeType<Objects>
        + MakeType<TailBytes>,
> represent::MakeWith<M> for Map
{
    fn make_with(maker: &mut M) -> Result<Self, <M as represent::Maker>::Error> {
        let header = maker.make()?;
        //dbg!(&header);
        let variables = maker.make()?;
        //dbg!(&variables);
        let tiles = maker.make()?;
        let scripts = maker.make()?;
        //dbg!(&scripts);
        let objects = maker.make()?;
        //dbg!(&objects);
        let tail = maker.make()?;
        Ok(Self {
            header,
            variables,
            tiles,
            scripts,
            objects,
            tail,
        })
    }
}

impl Map {
    pub fn header(&self) -> &Header {
        &self.header
    }

    pub fn variables(&self) -> &Variables {
        &self.variables
    }

    pub fn tiles(&self) -> &Tiles {
        &self.tiles
    }

    pub fn scripts(&self) -> &Scripts {
        &self.scripts
    }

    pub fn objects(&self) -> &Objects {
        &self.objects
    }

    pub fn tail(&self) -> &[u8] {
        &self.tail.0
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Hex(ToDo<i32>);

type Unknown<T> = ToDo<T>;
type Unused<T> = ToDo<T>;

pub fn parse_map<C: GetProto>(path: &Path, context: C) -> Result<Map, reader::F2ReaderError> {
    let bytes = std::fs::read(path).unwrap();
    reader::F2Reader::read(&bytes, context)
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
mod assert_makeable {
    use f2_common_format::{reader::F2Reader, GetProto, ProtoInfo};
    use represent::{MakeType, MakeWith};

    use crate::objects::{InventoryObject, LevelObject, ObjectKind, Objects};

    struct Context;
    struct Proto;
    impl GetProto for Context {
        type Proto = Proto;

        fn get_proto(&self, _proto_id: f2_common_format::Pid) -> Option<&Self::Proto> {
            unimplemented!()
        }
    }
    impl ProtoInfo for Proto {
        fn sub_type(&self) -> Option<u32> {
            unimplemented!()
        }
    }

    fn make_with<'a, T: MakeWith<F2Reader<'a, Context>>>() -> T {
        unimplemented!()
    }

    fn make_type<'a, T>() -> T
    where
        F2Reader<'a, Context>: MakeType<T>,
    {
        unimplemented!()
    }

    fn assert_object_types() {
        let _: [ObjectKind; 2] = [make_type(), make_with()];
        let _: [LevelObject; 2] = [make_type(), make_with()];
        let _: [InventoryObject; 2] = [make_type(), make_with()];
        let _: [Objects; 2] = [make_type(), make_with()];
    }
}

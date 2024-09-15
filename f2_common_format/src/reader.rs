use std::fmt;

use represent::{MakeType, MakeWith, Maker};
use represent_extra::{
    generics::{
        slots::{SlotLoadError, Slots},
        HasValue,
    },
    impl_analyzer,
    traits::{BytesLeft, MakeBlob},
};

use crate::{
    sniffer::{F2SniffError, F2Sniffer, Sniffer},
    Pid,
};

pub const SLOT_OBJECT_TYPE: usize = 0;
pub const SLOT_SUB_TYPE: usize = 1;
pub const SLOT_SCRIPT_TYPE: usize = 2;
pub const SLOT_INVENTORY_COUNT: usize = 3;

pub struct F2Reader<'a, C> {
    sniffer: F2Sniffer<'a>,
    slots: Vec<Slots>,
    context: C,
}

pub struct F2Context;

impl<'a, C> HasValue<F2Context> for F2Reader<'a, C> {
    type Value = C;

    fn give_value(&self) -> &C {
        &self.context
    }

    fn give_value_mut(&mut self) -> &mut C {
        &mut self.context
    }
}

impl<'a, C> F2Reader<'a, C> {
    pub fn read<T: MakeWith<Self>>(data: &'a [u8], context: C) -> Result<T, F2ReaderError> {
        let sniffer = F2Sniffer::new(data);
        let mut this = Self {
            sniffer,
            slots: vec![Slots::default()],
            context,
        };
        let res = this.make();
        if res.is_ok() {
            let bytes_left = this.sniffer.len();
            if bytes_left > 0 {
                return Err(F2ReaderError::BytesLeft(bytes_left));
            }
        }
        res
    }
}

#[derive(Debug)]
pub enum F2ReaderError {
    BytesLeft(usize),
    SniffError(F2SniffError),
    TryFromPrimitive(String),
    SlotLoadError(SlotLoadError<String>),
    InvalidObjectType,
    InvalidScriptType,
    ProtoNotFound(Pid),
    Validation(String),
}
impl<E: fmt::Debug> From<SlotLoadError<E>> for F2ReaderError {
    fn from(value: SlotLoadError<E>) -> Self {
        let err = match value {
            SlotLoadError::EmptySlot => SlotLoadError::EmptySlot,
            SlotLoadError::TryFrom(from) => SlotLoadError::TryFrom(format!("{:?}", from)),
        };
        Self::SlotLoadError(err)
    }
}
impl F2ReaderError {
    pub fn try_from_primitive<T: fmt::Debug>(value: T) -> Self {
        Self::TryFromPrimitive(format!("{:?}", value))
    }
}

impl_analyzer!(['a, C] for F2Reader<'a, C>);

impl<'a, C> Maker for F2Reader<'a, C> {
    type Error = F2ReaderError;
}

impl<'a, C, T: MakeWith<Self>> MakeType<T> for F2Reader<'a, C> {
    fn make_type(&mut self) -> Result<T, Self::Error> {
        T::make_with(self)
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct SlotsSpace<T>(pub T);

impl<'a, C, T> MakeType<SlotsSpace<T>> for F2Reader<'a, C>
where
    Self: MakeType<T>,
{
    fn make_type(&mut self) -> Result<SlotsSpace<T>, <Self as Maker>::Error> {
        self.slots.push(Default::default());
        let res = self.make().map(SlotsSpace);
        self.slots.pop();
        res
    }
}

impl<'a, C> HasValue<Slots> for F2Reader<'a, C> {
    type Value = Slots;

    fn give_value(&self) -> &Slots {
        self.slots.last().expect("last slots")
    }

    fn give_value_mut(&mut self) -> &mut Slots {
        self.slots.last_mut().expect("last slots")
    }
}

macro_rules! make_pod {
    ($($typ:ty),*$(,)?) => {
        $(
            impl<'a, C> MakeWith<F2Reader<'a, C>> for Pod<$typ> {
                fn make_with(maker: &mut F2Reader<'a, C>) -> Result<Pod<$typ>, F2ReaderError> {
                    use $crate::sniffer::Sniffer;
                    Ok(Pod(maker.sniffer.sniff_pod_reverse().map_err(F2ReaderError::SniffError)?))
                }
            }
            impl From<Pod<$typ>> for $typ {
                fn from(pod: Pod<$typ>) -> Self {
                    pod.0
                }
            }
        )*
    };
}
make_pod!(u8, u16, u32, i32);

/// Single big-endian number
#[derive(Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Pod<T>(pub T);

impl<T: fmt::Debug> fmt::Debug for Pod<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

macro_rules! make_primitive_enum  {
    ($($ty:ty),+$(,)?) => {
        $(
            impl<'a, C> represent::MakeType<$ty> for $crate::reader::F2Reader<'a, C> {
                fn make_type(&mut self) -> Result<$ty, $crate::reader::F2ReaderError> {
                    use num_enum::TryFromPrimitive;
                    let pod: $crate::reader::Pod<<$ty as TryFromPrimitive>::Primitive> = self.make_type()?;
                    <$ty>::try_from_primitive(pod.0).map_err($crate::reader::F2ReaderError::try_from_primitive)
                }
            }
            impl From<$ty> for u32 {
                fn from(value: $ty) -> Self {
                    value as _
                }
            }
        )+
    };
}
pub(crate) use make_primitive_enum;

/// Opaque type with unknown endianness
#[derive(Clone, Copy)]
//#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct ToDo<T>(T);

#[cfg(feature = "serde")]
mod impl_serde {
    use super::*;

    macro_rules! delegate_serde {
        ($($ty:ty)*) => {
            $(
                impl<'de> serde::Deserialize<'de> for ToDo<$ty> {
                    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                    where
                        D: serde::Deserializer<'de>,
                    {
                        Ok(Self(<$ty>::deserialize(deserializer)?))
                    }
                }

                impl serde::Serialize for ToDo<$ty> {
                    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                    where
                        S: serde::Serializer,
                    {
                        self.0.serialize(serializer)
                    }
                }
            )*
        };
    }
    delegate_serde!(u8 u16 u32 i32);

    impl<'de, T: serde::Deserialize<'de>, const LEN: usize> serde::Deserialize<'de> for ToDo<[T; LEN]> {
        fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de>,
        {
            Ok(Self(
                <[T; LEN] as serde_big_array::BigArray<'de, T>>::deserialize(deserializer)?,
            ))
        }
    }

    impl<T: serde::Serialize, const LEN: usize> serde::Serialize for ToDo<[T; LEN]> {
        fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer,
        {
            <[T; LEN] as serde_big_array::BigArray<'_, T>>::serialize(&self.0, serializer)
        }
    }
}

impl<T: fmt::Debug> fmt::Debug for ToDo<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl<'a, C, T: bytemuck::Pod + Copy> MakeWith<F2Reader<'a, C>> for ToDo<T> {
    fn make_with(maker: &mut F2Reader<'a, C>) -> Result<ToDo<T>, F2ReaderError> {
        Ok(ToDo(
            maker
                .sniffer
                .sniff_pod_reverse()
                .map_err(F2ReaderError::SniffError)?,
        ))
    }
}

impl<'a, C, T> MakeWith<F2Reader<'a, C>> for Option<T>
where
    F2Reader<'a, C>: MakeType<T, Error = F2ReaderError>,
{
    fn make_with(maker: &mut F2Reader<'a, C>) -> Result<Option<T>, F2ReaderError> {
        if maker.sniffer.is_empty() {
            Ok(None)
        } else {
            maker.make().map(Some)
        }
    }
}

impl<'a, C> MakeBlob for F2Reader<'a, C> {
    fn make_blob<T: bytemuck::Pod>(&mut self, len: usize) -> Result<Vec<T>, F2ReaderError> {
        self.sniffer
            .sniff_pod_vec_reverse(len)
            .map_err(F2ReaderError::SniffError)
    }
}

impl<'a, C> BytesLeft for F2Reader<'a, C> {
    fn bytes_left(&self) -> usize {
        self.sniffer.len()
    }
}

#[cfg(debug_assertions)]
#[allow(dead_code)]
mod assert_makeable {
    use represent::{MakeType, MakeWith};
    use represent_extra::{
        generics::length::LenConst,
        typedefs::{BigArrSlot, BigStaticArr, StaticStr},
    };

    use super::F2Reader;

    fn make_with<'a, T: MakeWith<F2Reader<'a, ()>>>() -> T {
        unimplemented!()
    }

    fn make_type<'a, T>() -> T
    where
        F2Reader<'a, ()>: MakeType<T>,
    {
        unimplemented!()
    }

    fn assert_container_types() {
        let _: [LenConst<16>; 2] = [make_type(), make_with()];
        let _: [BigStaticArr<u8, 16>; 2] = [make_type(), make_with()];
        let _: [BigStaticArr<i32, 44>; 2] = [make_type(), make_with()];
        let _: [StaticStr<16>; 2] = [make_type(), make_with()];
        let _: [BigArrSlot<u8, 0>; 2] = [make_type(), make_with()];
    }
}

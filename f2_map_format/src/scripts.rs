use f2_common_format::{
    reader::{F2ReaderError, Pod, SlotsSpace, ToDo, SLOT_SCRIPT_TYPE},
    ScriptType, Sid,
};
use represent::{MakeType, MakeWith, Maker, VisitWith};
use represent_extra::generics::{collections::RepeatExt, length::LenConst, slots::Load};

use crate::{Unknown, Unused};

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Scripts {
    sequences: RepeatExt<Sequence, LenConst<5>>,
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Sequence {
    scripts: Vec<Script>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Script {
    /// PID of the script.
    pid: Sid,
    slot: ScriptSlot,
}

#[derive(MakeWith, VisitWith)]
#[alt(ty = "Load<ScriptType, SLOT_SCRIPT_TYPE>")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ScriptSlot {
    #[alt("Load(ScriptType::Invalid)")]
    Garbage(Unused<[u32; 15]>),
    #[alt("_")]
    Body(ScriptBody),
}

impl std::fmt::Debug for ScriptSlot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Garbage(arg0) => f.debug_tuple("Garbage").finish(),
            Self::Body(arg0) => f.debug_tuple("Body").field(arg0).finish(),
        }
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct ScriptBody {
    /// Next script. Unused.
    next_script: Unused<i32>,
    kind: ScriptKind,
    /// Script flags (0 in maps, value in saves).
    flags: ToDo<u32>,
    /// Script id.
    /// Script filename is found in LST file script.lst at index id.
    script_id: ToDo<u32>,
    unknown_5: Unknown<i32>,
    script_oid: ToDo<u32>,
    /// Local var offset (-1 in maps, value  on saves).
    local_var_offset: ToDo<i32>,
    /// Num local vars (0 in maps, value in saves).
    num_local_vars: ToDo<u32>,
    unknown: Unknown<[i32; 8]>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[alt(
    ty = "Load<ScriptType, SLOT_SCRIPT_TYPE>",
    err = "F2ReaderError",
    default = "Err(F2ReaderError::InvalidScriptType.into())"
)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ScriptKind {
    #[alt("Load(ScriptType::System)")]
    System,
    #[alt("Load(ScriptType::Spatial)")]
    Spatial(SpatialScript),
    #[alt("Load(ScriptType::Timer)")]
    Timer(TimerScript),
    #[alt("Load(ScriptType::Item)")]
    Item,
    #[alt("Load(ScriptType::Critter)")]
    Critter,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct SpatialScript {
    /// Spatial script hex. First two bytes are elevation:
    /// 0x0000 - 1
    /// 0x2000 - 2
    /// 0x4000 - 3
    hex: ToDo<u32>,
    /// Spatial script radius.
    radius: ToDo<i32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct TimerScript {
    /// Timer script time
    timer: ToDo<i32>,
}

#[derive(Debug)]
enum Error {
    Check { count: u32, check: u32 },
}

impl From<Error> for F2ReaderError {
    fn from(value: Error) -> Self {
        match value {
            Error::Check { count, check } => {
                Self::Validation(format!("Wrong number of scripts: {check} != {count}"))
            }
        }
    }
}

impl<M: Maker + MakeType<Pod<u32>> + MakeType<SlotsSpace<Script>> + MakeType<Unused<[u32; 16]>>>
    MakeWith<M> for Sequence
where
    Error: Into<M::Error>,
{
    fn make_with(maker: &mut M) -> Result<Self, M::Error> {
        let Pod(count) = maker.make()?;
        let mut scripts = vec![];

        let loops = (count + 15) / 16;
        let mut check: u32 = 0;

        for _ in 0..loops {
            for _ in 0..16 {
                let script: SlotsSpace<Script> = maker.make()?;
                scripts.push(script.0);
            }
            let Pod(diff) = maker.make()?;
            check += diff;
            let _unknown: Pod<u32> = maker.make()?;
        }
        if check != count {
            Err(Error::Check { count, check }.into())
        } else {
            Ok(Self { scripts })
        }
    }
}

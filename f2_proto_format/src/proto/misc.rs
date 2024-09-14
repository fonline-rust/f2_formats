use f2_common_format::{ObjectPid, MiscSubType};
use represent_derive::{MakeWith, Visit};

use super::ToDo;

#[derive(Debug, MakeWith, Visit)]
pub struct Misc {
    unknown: ToDo<u32>,
}

impl Misc {
    pub(super) fn sub_type(&self, pid: &ObjectPid) -> MiscSubType {
        match pid.id() {
            16..=23 => MiscSubType::ExitGrid,
            _ => MiscSubType::Generic,
        }
    }
}

use f2_common_format::{MiscSubType, ObjectPid};
use represent::{MakeWith, VisitWith};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
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

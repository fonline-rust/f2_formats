use represent::{MakeWith, VisitWith};
use represent_extra::typedefs::BigArrSlot;

use crate::slots;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Variables {
    global_vars: BigArrSlot<i32, { slots::NUM_GLOBAL_VARS }>,
    local_vars: BigArrSlot<i32, { slots::NUM_LOCAL_VARS }>,
}
impl Variables {
    pub fn global_vars(&self) -> &[i32] {
        &self.global_vars.0
    }

    pub fn local_vars(&self) -> &[i32] {
        &self.local_vars.0
    }
}

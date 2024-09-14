use fo_net_protocol::msg::ArrSlot;
use represent_derive::{MakeWith, Visit};

use crate::slots;

#[derive(Debug, MakeWith, Visit)]
pub struct Variables {
    global_vars: ArrSlot<i32, {slots::NUM_GLOBAL_VARS}>,
    local_vars: ArrSlot<i32, {slots::NUM_LOCAL_VARS}>,
}
impl Variables {
    pub fn global_vars(&self) -> &[i32] {
        &self.global_vars.0
    }
    pub fn local_vars(&self) -> &[i32] {
        &self.local_vars.0
    }
}

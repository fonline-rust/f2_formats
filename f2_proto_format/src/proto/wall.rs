use represent::{MakeWith, VisitWith};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Wall {
    wall_light_flags: ToDo<u16>,
    action_flags: ToDo<u16>,
    script_id: ToDo<u32>,
    material_id: ToDo<u32>,
}

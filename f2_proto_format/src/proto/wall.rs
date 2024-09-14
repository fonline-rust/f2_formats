use represent_derive::{MakeWith, Visit};

use super::ToDo;

#[derive(Debug, MakeWith, Visit)]
pub struct Wall {
    wall_light_flags: ToDo<u16>,
    action_flags: ToDo<u16>,
    script_id: ToDo<u32>,
    material_id: ToDo<u32>,
}

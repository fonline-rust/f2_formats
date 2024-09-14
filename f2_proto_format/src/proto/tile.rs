use represent_derive::{MakeWith, Visit};

use super::ToDo;

#[derive(Debug, MakeWith, Visit)]
pub struct Tile {
    material_id: ToDo<u32>,
}

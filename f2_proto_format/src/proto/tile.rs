use represent::{MakeWith, VisitWith};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Tile {
    material_id: ToDo<u32>,
}

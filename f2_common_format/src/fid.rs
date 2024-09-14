use represent::Maker;

use crate::reader::{F2Reader, F2ReaderError, Pod};

#[derive(Debug)]
pub struct Fid {
    ty: FrmType,
    lst_index: u16,
    id1: u8,
    id2: u8,
    id3: u8,
}

#[derive(Clone, Copy)]
struct Mask {
    shift: u32,
    mask: u32,
}
impl Mask {
    fn apply(self, value: u32) -> u32 {
        (value & self.mask) >> self.shift
    }
}
// TODO: make macro
const fn calc_fid_masks() -> [Mask; 4 + 1] {
    let mut masks = [Mask { shift: 32, mask: 0 }; 5];
    let mut i = 0u32;
    let mut mask = 1;
    while i < 32 {
        let part = match i {
            0..=11 => 0,
            12..=15 => 1,
            16..=23 => 2,
            24..=27 => 3,
            28..=31 => 4,
            _ => unreachable!(),
        };
        if masks[part].shift > i {
            masks[part].shift = i;
        }
        masks[part].mask |= mask;
        mask <<= 1;
        i += 1;
    }
    masks
}
const MASKS: [Mask; 5] = calc_fid_masks();

impl<'a, C> represent::MakeType<Fid> for F2Reader<'a, C> {
    fn make_type(&mut self) -> Result<Fid, F2ReaderError> {
        use num_enum::TryFromPrimitive;
        let Pod(raw): Pod<u32> = self.make()?;

        let lst_index = MASKS[0].apply(raw) as u16;
        let id1 = MASKS[1].apply(raw) as u8;
        let id2 = MASKS[2].apply(raw) as u8;
        let ty_byte = MASKS[3].apply(raw) as u8;
        let id3 = MASKS[4].apply(raw) as u8;

        let ty = FrmType::try_from_primitive(ty_byte).map_err(F2ReaderError::try_from_primitive)?;

        Ok(Fid {
            ty,
            lst_index,
            id1,
            id2,
            id3,
        })
    }
}

#[derive(Debug, num_enum::TryFromPrimitive)]
#[repr(u8)]
pub enum FrmType {
    Item = 0x0,
    Critter = 0x1,
    Scenery = 0x2,
    Wall = 0x3,
    Tile = 0x4,
    Misc = 0x5,
    Interface = 0x6,
    Inventory = 0x7,
    Heads = 0x8,
    Background = 0x9,
    Skilldex = 0xA,
    Invalid = 0xF,
}

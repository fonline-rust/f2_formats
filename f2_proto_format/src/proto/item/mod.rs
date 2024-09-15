use f2_common_format::{
    reader::{Pod, SLOT_SUB_TYPE},
    Fid, ItemSubType, Perk, Pid,
};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::{Load, Store};

use super::ToDo;

mod drug;
mod weapon;

use self::{drug::Drug, weapon::Weapon};

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Item {
    flags: ToDo<u32>,
    script_id: Pid,
    ty: Store<ItemSubType, SLOT_SUB_TYPE>,
    material_id: ToDo<u32>,
    /// Volume in containers
    size: Pod<u32>,
    weight: Pod<u32>,
    cost: Pod<u32>,
    /// FRM ID for the item in inventory
    inv_frm_id: Fid,
    /// Sound related to the item: when picked up from the ground, opening a container, etc
    sound_id: ToDo<u8>,
    kind: ItemKind,
}
impl Item {
    pub fn sub_type(&self) -> ItemSubType {
        self.ty.inner
    }
}

#[derive(Debug, MakeWith, VisitWith)]
#[alt(ty = "Load<ItemSubType, SLOT_SUB_TYPE>")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
enum ItemKind {
    #[alt("Load(ItemSubType::Armor)")]
    Armor(Armor),
    #[alt("Load(ItemSubType::Container)")]
    Container(Container),
    #[alt("Load(ItemSubType::Drug)")]
    Drug(Drug),
    #[alt("Load(ItemSubType::Weapon)")]
    Weapon(Weapon),
    #[alt("Load(ItemSubType::Ammo)")]
    Ammo(Ammo),
    #[alt("Load(ItemSubType::Misc)")]
    Misc(Misc),
    #[alt("Load(ItemSubType::Key)")]
    Key(Key),
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Armor {
    armor_class: Pod<u32>,
    resistance: EveryDamage,
    threshold: EveryDamage,
    /// Note: the object may have any perk, but not all will work.
    perk: Perk,
    male_frm_id: Fid,
    female_frm_id: Fid,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct EveryDamage {
    normal: Pod<u32>,
    laser: Pod<u32>,
    fire: Pod<u32>,
    plasma: Pod<u32>,
    electrical: Pod<u32>,
    emp: Pod<u32>,
    explosion: Pod<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Container {
    /// Max Size (how much it can contain)
    max_size: Pod<u32>,
    /// Open Flags:
    /// 0x00000001 - Cannot Pick Up (implies Magic Hands Grnd!)
    /// 0x00000008 - Magic Hands Grnd (reach down to open/close)
    open_flags: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Ammo {
    /// Ammo type
    /// Values: see proto.msg, starting with the line 300
    caliber: ToDo<u32>,
    /// The number of rounds in a magazine
    quantity: Pod<u32>,
    armor_class_modifier: ToDo<i32>,
    damage_resistense_modifier: ToDo<i32>,
    ammo_damage_multiplier: ToDo<u32>,
    ammo_damage_divisor: ToDo<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Misc {
    power_pid: Pid,
    /// Values: see proto.msg, starting with the line 300
    power_ty: ToDo<u32>,
    /// The maximum number of charges
    charges: Pod<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
struct Key {
    /// always 0xFFFFFFFF
    key_code: ToDo<u32>,
}

use f2_common_format::{
    reader::{Pod, ToDo, SLOT_SUB_TYPE},
    ItemSubType, Pid,
};
use represent::{MakeWith, VisitWith};
use represent_extra::generics::slots::Load;

#[derive(Debug, MakeWith, VisitWith)]
#[alt(ty = "Load<ItemSubType, SLOT_SUB_TYPE>")]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum Item {
    #[alt("Load(ItemSubType::Armor)")]
    Armor,
    #[alt("Load(ItemSubType::Container)")]
    Container,
    #[alt("Load(ItemSubType::Drug)")]
    Drug,
    #[alt("Load(ItemSubType::Weapon)")]
    Weapon(Weapon),
    #[alt("Load(ItemSubType::Ammo)")]
    Ammo(Ammo),
    #[alt("Load(ItemSubType::Misc)")]
    Misc(MiscItem),
    #[alt("Load(ItemSubType::Key)")]
    Key(Key),
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Weapon {
    /// Ammo count. Amount of ammunition loaded in this weapon.
    ammo_quantity: Pod<u32>,
    /// Ammo count. Amount of ammunition loaded in this weapon.
    ammo_id: Pid,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Ammo {
    /// Amount of ammo in magazine. Number of bullets or charges in this magazine.
    quantity: Pod<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct MiscItem {
    charges: Pod<u32>,
}

#[derive(Debug, MakeWith, VisitWith)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Key {
    key_code: ToDo<u32>,
}

use fo_net_protocol::generics::slots::Load;
use represent_derive::{MakeWith, Visit};
use f2_common_format::{reader::{SLOT_SUB_TYPE, Pod, ToDo}, ItemSubType, Pid};

#[derive(Debug, MakeWith, Visit)]
#[alt(ty = "Load<ItemSubType, SLOT_SUB_TYPE>")]
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

#[derive(Debug, MakeWith, Visit)]
pub struct Weapon {
    /// Ammo count. Amount of ammunition loaded in this weapon. 
    ammo_quantity: Pod<u32>,
    /// Ammo count. Amount of ammunition loaded in this weapon. 
    ammo_id: Pid, 
}

#[derive(Debug, MakeWith, Visit)]
pub struct Ammo {
    /// Amount of ammo in magazine. Number of bullets or charges in this magazine. 
    quantity: Pod<u32>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct MiscItem {
    charges: Pod<u32>,
}

#[derive(Debug, MakeWith, Visit)]
pub struct Key {
    key_code: ToDo<u32>,
}

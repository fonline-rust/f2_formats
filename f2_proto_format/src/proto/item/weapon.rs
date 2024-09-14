use f2_common_format::{reader::Pod, Perk, Pid};
use represent::{MakeWith, VisitWith};

use super::ToDo;

#[derive(Debug, MakeWith, VisitWith)]
pub struct Weapon {
    /// Anim Code
    /// Which frameset is used for characters wielding that weapon.
    /// Values:
    /// 0x00 - None (A)
    /// 0x01 - Knife (D)
    /// 0x02 - Club (E)
    /// 0x03 - Sledgehammer (F)
    /// 0x04 - Spear (G)
    /// 0x05 - Pistol (H)
    /// 0x06 - SMG (I)
    /// 0x07 - Rifle (J)
    /// 0x08 - Big Gun (K)
    /// 0x09 - Minigun (L)
    /// 0x0A - Rocket Launcher (M)
    animation_code: ToDo<u32>,
    damage: Damage,
    /// The maximum distance for Primary Attack
    max_range_primary: ToDo<u32>,
    /// The maximum distance for Secondary Attack
    max_range_secondary: ToDo<u32>,
    /// Should be ObjectType::Misc
    projectile_proto_id: Pid,
    minimal_strenght: ToDo<u32>,
    /// The number of AP for Primary Attack
    ap_cost_primary: ToDo<u32>,
    /// The number of AP for Secondary Attack
    ap_cost_secondary: ToDo<u32>,
    /// Number of the list of critical failures possible for this weapon.
    crit_fail: ToDo<u32>,
    /// Note: the weapons can be have any perk, but not all will work.
    perk: Perk,
    /// The number of rounds fired in a burst attack.
    rounds: Pod<u32>,
    /// Ammo type
    /// Values: see proto.msg, starting with the line 300
    caliber: ToDo<u32>,
    ammo_proto_id: Pid,
    /// Size of the magazine
    max_ammo: Pod<u32>,
    /// Sound effects for the weapon: shooting, reloading, etc.
    sound_id: ToDo<u8>,
}

#[derive(Debug, MakeWith, VisitWith)]
struct Damage {
    min: ToDo<u32>,
    max: ToDo<u32>,
    /// Dmg Type
    /// (see proto.msg, starting with the line 250)
    /// 0 - Normal
    /// 1 - Laser
    /// 2 - Fire
    /// 3 - Plasma
    /// 4 - Electrical
    /// 5 - EMP
    /// 6 - Explosive
    ty: ToDo<u32>,
}

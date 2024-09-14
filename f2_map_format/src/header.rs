
use f2_common_format::{MapVersion, reader::{Pod, ToDo}};
use fo_net_protocol::{msg::StaticStr, generics::slots::Store};
use represent_derive::{MakeWith, Visit};

use crate::{Hex, slots, Unused};

#[derive(Debug, MakeWith, Visit)]
pub struct Header {
    /// Map version.
    version: MapVersion,
    /// Map filename.
    filename: StaticStr<16>,
    /// Default player position. The default hex grid that the player will start in when the map is entered, if not overridden. 
    default_player_position: Hex,
    /// Default map elevation. The default map elevation for the player to start in when the map is entered, if not overridden. 
    /// Range: 0..=2
    default_map_elevation: Pod<i32>,
    /// Default player orientation. The default orientation the player is facing when the map is entered.
    /// Range: 0..=5
    default_map_orientation: Pod<i32>,
    /// Number of local variables stored in map. 
    num_local_vars: Store<Pod<u32>, {slots::NUM_LOCAL_VARS}>,
    /// Script id for this map. Value of -1 means no map. Text string is found in MSG file scrname.msg at index [id + 101].
    script_id: ToDo<i32>,
    /// /// Map flags.
    /// - If (flag & 0x1) != 0 then the map is a savegame map (.SAV).
    /// - If (flag & 0x2) == 0 then the map has an elevation at level 0.
    /// - If (flag & 0x4) == 0 then the map has an elevation at level 1.
    /// - If (flag & 0x8) == 0 then the map has an elevation at level 2.
    flags: Store<Pod<u32>, {slots::MAP_FLAGS}>,
    /// Map darkness.
    darkness: Pod<i32>,
    /// Number of global variables stored in map.
    num_global_vars: Store<Pod<u32>, {slots::NUM_GLOBAL_VARS}>,
    /// Map Id.
    /// - Fallout 1: Map filename found in map.msg
    /// - Fallout 2: Map details found in data/maps.txt in section [Map id]
    map_id: Pod<i32>,
    /// Time since the epoch.
    /// Number of time ticks since the epoch. A time tick is equivalent to 0.1 seconds in game time.
    /// - The epoch for Fallout 1 is "5 December 2161 00:00am"
    /// - The epoch for Fallout 2 is "25 July 2241 00:00am"
    time_since_the_epoch: Pod<u32>,
    unused: Unused<[i32; 44]>,
}

impl Header {
    pub fn filename(&self) -> Option<&str> {
        std::str::from_utf8(&self.filename.0.0).ok()
    }
}

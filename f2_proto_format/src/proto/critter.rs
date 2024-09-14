use f2_common_format::{Fid, reader::Pod};
use represent_derive::{MakeWith, Visit};

use super::ToDo;

#[derive(Debug, MakeWith, Visit)]
pub struct Critter {
    flags_ext: ToDo<u32>,
    /// Script ID
    /// Format: 0x0Y00XXXX
    /// Y-type specifier (1-spatial, 2-items, 3 - scenery, 4-critters)
    /// XXXX-room in scripts.lst
    /// If the value is 0xFFFFFFFF, there is no script 
    script_id: ToDo<u32>,
    /// Head FID (for talking heads) 
    head_frm_id: Fid,
    /// AI Packet
    /// Format: 0x00000XXX XXX-see ai.txt, parameter packet_num Note: Some numbers can be drawn from packages and aibdymsg.txt aigenmsg.txt. Similarly ai.txt. 
    ai_packet: ToDo<u32>,
    team_num: ToDo<u32>,
    critter_flags: ToDo<u32>,
    params: ToDo<[u32; 91]>,
    damage_type: Option<ToDo<u32>>,
}

use phf_macros::phf_map;
pub static player_offsets: phf::Map<&'static str, u32> = phf_map! {

    "armor" => 0x10c, 
    "health" => 0x108, 
    "ar_mag" => 0x138,
    "ar_bullets" => 0x15C,
    "pistol_bullets" => 0x148,
    "pistol_actions" => 0x16C, // 160 = shooting, 1400 = reloading, 1647 = empty_clip_reloading 
    "recoil" => 0x44, // `no_recoil` function has the NOP instructions to give player no recoil
    "grenades" => 0x160,
    "player_state" => 0x78, // 262144 = noclip state, 
    "username" => 0x225,
 };
 
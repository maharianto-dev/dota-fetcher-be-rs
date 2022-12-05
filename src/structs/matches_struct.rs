use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, Serialize)]
pub struct MatchHistoryByPlayerIdStruct {
    match_id: u64,
    player_slot: u32,
    radiant_win: bool,
    duration: u32,
    game_mode: u32,
    lobby_type: u32,
    hero_id: u32,
    start_time: u32,
    version: Option<u32>,
    kills: u32,
    deaths: u32,
    assists: u32,
    skill: Option<u32>,
    average_rank: u32,
    leaver_status: u32,
    party_size: u32,
}

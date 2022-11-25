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

impl MatchHistoryByPlayerIdStruct {
    pub fn create_error_type() -> Self {
        MatchHistoryByPlayerIdStruct {
            match_id: 0,
            player_slot: 0,
            radiant_win: false,
            duration: 0,
            game_mode: 0,
            lobby_type: 0,
            hero_id: 0,
            start_time: 0,
            version: Some(0),
            kills: 0,
            deaths: 0,
            assists: 0,
            skill: Some(0),
            average_rank: 0,
            leaver_status: 0,
            party_size: 0,
        }
    }
}

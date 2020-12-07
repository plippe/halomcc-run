use strum_macros::EnumIter;
use time::time;

use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
pub enum Halo3Mission {
    Arrival,
    SierraOneOneSeven,
    CrowsNest,
    TvasoHighway,
    TheStorm,
    Floodgate,
    TheArk,
    TheCovenant,
    Cortana,
    Halo,
    Epilogue,
}

impl Halo3Mission {
    pub fn to_mission(&self) -> Mission {
        Mission::Halo3(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::Arrival => MissionProperties { game_id: 3, id: 1, name: "Arrival".to_string(), par_time: None, par_score: None },
            Self::SierraOneOneSeven => MissionProperties { game_id: 3, id: 2, name: "Sierra 117".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(13_000) },
            Self::CrowsNest => MissionProperties { game_id: 3, id: 3, name: "Crow’s Nest".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(19_000) },
            Self::TvasoHighway => MissionProperties { game_id: 3, id: 4, name: "Tvaso Highway".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(21_000) },
            Self::TheStorm => MissionProperties { game_id: 3, id: 5, name: "The Storm".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(15_000) },
            Self::Floodgate => MissionProperties { game_id: 3, id: 6, name: "Floodgate".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(25_000) },
            Self::TheArk => MissionProperties { game_id: 3, id: 7, name: "The Ark".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::TheCovenant => MissionProperties { game_id: 3, id: 8, name: "The Covenant".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::Cortana => MissionProperties { game_id: 3, id: 9, name: "Cortana".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            Self::Halo => MissionProperties { game_id: 3, id: 10, name: "Halo".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(24_000) },
            Self::Epilogue => MissionProperties { game_id: 3, id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

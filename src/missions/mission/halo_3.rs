use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
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

impl From<Halo3Mission> for Mission {
    fn from(mission: Halo3Mission) -> Self {
        Self::Halo3(mission)
    }
}

impl From<Halo3Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: Halo3Mission) -> Self {
        match mission {
            Halo3Mission::Arrival => MissionProperties { game_id: Game::Halo3.id(), id: 1, name: "Arrival".to_string(), par_time: None, par_score: None },
            Halo3Mission::SierraOneOneSeven => MissionProperties { game_id: Game::Halo3.id(), id: 2, name: "Sierra 117".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(13_000) },
            Halo3Mission::CrowsNest => MissionProperties { game_id: Game::Halo3.id(), id: 3, name: "Crow’s Nest".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(19_000) },
            Halo3Mission::TvasoHighway => MissionProperties { game_id: Game::Halo3.id(), id: 4, name: "Tvaso Highway".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(21_000) },
            Halo3Mission::TheStorm => MissionProperties { game_id: Game::Halo3.id(), id: 5, name: "The Storm".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(15_000) },
            Halo3Mission::Floodgate => MissionProperties { game_id: Game::Halo3.id(), id: 6, name: "Floodgate".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(25_000) },
            Halo3Mission::TheArk => MissionProperties { game_id: Game::Halo3.id(), id: 7, name: "The Ark".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Halo3Mission::TheCovenant => MissionProperties { game_id: Game::Halo3.id(), id: 8, name: "The Covenant".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Halo3Mission::Cortana => MissionProperties { game_id: Game::Halo3.id(), id: 9, name: "Cortana".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            Halo3Mission::Halo => MissionProperties { game_id: Game::Halo3.id(), id: 10, name: "Halo".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(24_000) },
            Halo3Mission::Epilogue => MissionProperties { game_id: Game::Halo3.id(), id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

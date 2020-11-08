use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
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

impl From<&Halo3Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &Halo3Mission) -> Self {
        match mission {
            Halo3Mission::Arrival => MissionProperties::new(Game::Halo3, 1, "Arrival".to_string(), None, None),
            Halo3Mission::SierraOneOneSeven => MissionProperties::new(Game::Halo3, 2, "Sierra 117".to_string(), Some(time!(00:15:00)), Some(13_000)),
            Halo3Mission::CrowsNest => MissionProperties::new(Game::Halo3, 3, "Crow’s Nest".to_string(), Some(time!(00:20:00)), Some(19_000)),
            Halo3Mission::TvasoHighway => MissionProperties::new(Game::Halo3, 4, "Tvaso Highway".to_string(), Some(time!(00:20:00)), Some(21_000)),
            Halo3Mission::TheStorm => MissionProperties::new(Game::Halo3, 5, "The Storm".to_string(), Some(time!(00:15:00)), Some(15_000)),
            Halo3Mission::Floodgate => MissionProperties::new(Game::Halo3, 6, "Floodgate".to_string(), Some(time!(00:15:00)), Some(25_000)),
            Halo3Mission::TheArk => MissionProperties::new(Game::Halo3, 7, "The Ark".to_string(), Some(time!(00:20:00)), Some(25_000)),
            Halo3Mission::TheCovenant => MissionProperties::new(Game::Halo3, 8, "The Covenant".to_string(), Some(time!(00:20:00)), Some(25_000)),
            Halo3Mission::Cortana => MissionProperties::new(Game::Halo3, 9, "Cortana".to_string(), Some(time!(00:15:00)), Some(17_000)),
            Halo3Mission::Halo => MissionProperties::new(Game::Halo3, 10, "Halo".to_string(), Some(time!(00:20:00)), Some(24_000)),
            Halo3Mission::Epilogue => MissionProperties::new(Game::Halo3, 11, "Epilogue".to_string(), None, None),
        }
    }
}

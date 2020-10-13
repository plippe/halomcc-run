use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
pub enum Halo3Mission {
    SierraOneOneSeven,
    CrowsNest,
    TvasoHighway,
    TheStorm,
    Floodgate,
    TheArk,
    TheCovenant,
    Cortana,
    Halo,
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
            Halo3Mission::SierraOneOneSeven => MissionProperties::new(Game::Halo3, 1, "Sierra 117".to_string(), Some(time!(00:15:00))),
            Halo3Mission::CrowsNest => MissionProperties::new(Game::Halo3, 2, "Crow’s Nest".to_string(), Some(time!(00:20:00))),
            Halo3Mission::TvasoHighway => MissionProperties::new(Game::Halo3, 3, "Tvaso Highway".to_string(), Some(time!(00:20:00))),
            Halo3Mission::TheStorm => MissionProperties::new(Game::Halo3, 4, "The Storm".to_string(), Some(time!(00:15:00))),
            Halo3Mission::Floodgate => MissionProperties::new(Game::Halo3, 5, "Floodgate".to_string(), Some(time!(00:15:00))),
            Halo3Mission::TheArk => MissionProperties::new(Game::Halo3, 6, "The Ark".to_string(), Some(time!(00:20:00))),
            Halo3Mission::TheCovenant => MissionProperties::new(Game::Halo3, 7, "The Covenant".to_string(), Some(time!(00:20:00))),
            Halo3Mission::Cortana => MissionProperties::new(Game::Halo3, 8, "Cortana".to_string(), Some(time!(00:15:00))),
            Halo3Mission::Halo => MissionProperties::new(Game::Halo3, 9, "Halo".to_string(), Some(time!(00:20:00))),
        }
    }
}

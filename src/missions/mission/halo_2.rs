use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
pub enum Halo2Mission {
    CairoStation,
    Outskirts,
    Metropolis,
    TheArbiter,
    TheOracle,
    DeltaHalo,
    Regret,
    SacredIcon,
    QuarantineZone,
    Gravemind,
    Uprising,
    HighCharity,
    TheGreatJourney,
}

impl From<Halo2Mission> for Mission {
    fn from(mission: Halo2Mission) -> Self {
        Self::Halo2(mission)
    }
}

impl From<&Halo2Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &Halo2Mission) -> Self {
        match mission {
            Halo2Mission::CairoStation => MissionProperties::new(Game::Halo2, 1, "Cairo Station".to_string(), Some(time!(00:15:00))),
            Halo2Mission::Outskirts => MissionProperties::new(Game::Halo2, 2, "Outskirts".to_string(), Some(time!(00:15:00))),
            Halo2Mission::Metropolis => MissionProperties::new(Game::Halo2, 3, "Metropolis".to_string(), Some(time!(00:15:00))),
            Halo2Mission::TheArbiter => MissionProperties::new(Game::Halo2, 4, "The Arbiter".to_string(), Some(time!(00:15:00))),
            Halo2Mission::TheOracle => MissionProperties::new(Game::Halo2, 5, "The Oracle".to_string(), Some(time!(00:25:00))),
            Halo2Mission::DeltaHalo => MissionProperties::new(Game::Halo2, 6, "Delta Halo".to_string(), Some(time!(00:15:00))),
            Halo2Mission::Regret => MissionProperties::new(Game::Halo2, 7, "Regret".to_string(), Some(time!(00:15:00))),
            Halo2Mission::SacredIcon => MissionProperties::new(Game::Halo2, 8, "Sacred Icon".to_string(), Some(time!(00:15:00))),
            Halo2Mission::QuarantineZone => MissionProperties::new(Game::Halo2, 9, "Quarantine Zone".to_string(), Some(time!(00:15:00))),
            Halo2Mission::Gravemind => MissionProperties::new(Game::Halo2, 10, "Gravemind".to_string(), Some(time!(00:20:00))),
            Halo2Mission::Uprising => MissionProperties::new(Game::Halo2, 11, "Uprising".to_string(), Some(time!(00:15:00))),
            Halo2Mission::HighCharity => MissionProperties::new(Game::Halo2, 12, "High Charity".to_string(), Some(time!(00:15:00))),
            Halo2Mission::TheGreatJourney => MissionProperties::new(Game::Halo2, 13, "The Great Journey".to_string(), Some(time!(00:15:00))),
        }
    }
}

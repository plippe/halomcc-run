use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
pub enum Halo2Mission {
    TheHeretic,
    TheArmory,
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
            Halo2Mission::TheHeretic => MissionProperties::new(Game::Halo2, 1, "The Heretic".to_string(), None, None),
            Halo2Mission::TheArmory => MissionProperties::new(Game::Halo2, 2, "The Armory".to_string(), None, None),
            Halo2Mission::CairoStation => MissionProperties::new(Game::Halo2, 3, "Cairo Station".to_string(), Some(time!(00:15:00)), Some(14_000)),
            Halo2Mission::Outskirts => MissionProperties::new(Game::Halo2, 4, "Outskirts".to_string(), Some(time!(00:15:00)), Some(8_000)),
            Halo2Mission::Metropolis => MissionProperties::new(Game::Halo2, 5, "Metropolis".to_string(), Some(time!(00:15:00)), Some(9_000)),
            Halo2Mission::TheArbiter => MissionProperties::new(Game::Halo2, 6, "The Arbiter".to_string(), Some(time!(00:15:00)), Some(7_000)),
            Halo2Mission::TheOracle => MissionProperties::new(Game::Halo2, 7, "The Oracle".to_string(), Some(time!(00:25:00)), Some(16_000)),
            Halo2Mission::DeltaHalo => MissionProperties::new(Game::Halo2, 8, "Delta Halo".to_string(), Some(time!(00:15:00)), Some(10_000)),
            Halo2Mission::Regret => MissionProperties::new(Game::Halo2, 9, "Regret".to_string(), Some(time!(00:15:00)), Some(8_000)),
            Halo2Mission::SacredIcon => MissionProperties::new(Game::Halo2, 10, "Sacred Icon".to_string(), Some(time!(00:15:00)), Some(7_000)),
            Halo2Mission::QuarantineZone => MissionProperties::new(Game::Halo2, 11, "Quarantine Zone".to_string(), Some(time!(00:15:00)), Some(7_000)),
            Halo2Mission::Gravemind => MissionProperties::new(Game::Halo2, 12, "Gravemind".to_string(), Some(time!(00:20:00)), Some(11_000)),
            Halo2Mission::Uprising => MissionProperties::new(Game::Halo2, 13, "Uprising".to_string(), Some(time!(00:15:00)), Some(9_000)),
            Halo2Mission::HighCharity => MissionProperties::new(Game::Halo2, 14, "High Charity".to_string(), Some(time!(00:15:00)), Some(9_000)),
            Halo2Mission::TheGreatJourney => MissionProperties::new(Game::Halo2, 15, "The Great Journey".to_string(), Some(time!(00:15:00)), Some(8_000)),
        }
    }
}

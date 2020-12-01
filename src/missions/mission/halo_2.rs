use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
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

impl From<Halo2Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: Halo2Mission) -> Self {
        match mission {
            Halo2Mission::TheHeretic => MissionProperties { game_id: Game::Halo2.id(), id: 1, name: "The Heretic".to_string(), par_time: None, par_score: None },
            Halo2Mission::TheArmory => MissionProperties { game_id: Game::Halo2.id(), id: 2, name: "The Armory".to_string(), par_time: None, par_score: None },
            Halo2Mission::CairoStation => MissionProperties { game_id: Game::Halo2.id(), id: 3, name: "Cairo Station".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(14_000) },
            Halo2Mission::Outskirts => MissionProperties { game_id: Game::Halo2.id(), id: 4, name: "Outskirts".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
            Halo2Mission::Metropolis => MissionProperties { game_id: Game::Halo2.id(), id: 5, name: "Metropolis".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Halo2Mission::TheArbiter => MissionProperties { game_id: Game::Halo2.id(), id: 6, name: "The Arbiter".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Halo2Mission::TheOracle => MissionProperties { game_id: Game::Halo2.id(), id: 7, name: "The Oracle".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(16_000) },
            Halo2Mission::DeltaHalo => MissionProperties { game_id: Game::Halo2.id(), id: 8, name: "Delta Halo".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(10_000) },
            Halo2Mission::Regret => MissionProperties { game_id: Game::Halo2.id(), id: 9, name: "Regret".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
            Halo2Mission::SacredIcon => MissionProperties { game_id: Game::Halo2.id(), id: 10, name: "Sacred Icon".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Halo2Mission::QuarantineZone => MissionProperties { game_id: Game::Halo2.id(), id: 11, name: "Quarantine Zone".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Halo2Mission::Gravemind => MissionProperties { game_id: Game::Halo2.id(), id: 12, name: "Gravemind".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(11_000) },
            Halo2Mission::Uprising => MissionProperties { game_id: Game::Halo2.id(), id: 13, name: "Uprising".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Halo2Mission::HighCharity => MissionProperties { game_id: Game::Halo2.id(), id: 14, name: "High Charity".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Halo2Mission::TheGreatJourney => MissionProperties { game_id: Game::Halo2.id(), id: 15, name: "The Great Journey".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
        }
    }
}

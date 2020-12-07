use strum_macros::EnumIter;
use time::time;

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

impl Halo2Mission {
    pub fn to_mission(&self) -> Mission {
        Mission::Halo2(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::TheHeretic => MissionProperties { game_id: 2, id: 1, name: "The Heretic".to_string(), par_time: None, par_score: None },
            Self::TheArmory => MissionProperties { game_id: 2, id: 2, name: "The Armory".to_string(), par_time: None, par_score: None },
            Self::CairoStation => MissionProperties { game_id: 2, id: 3, name: "Cairo Station".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(14_000) },
            Self::Outskirts => MissionProperties { game_id: 2, id: 4, name: "Outskirts".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
            Self::Metropolis => MissionProperties { game_id: 2, id: 5, name: "Metropolis".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Self::TheArbiter => MissionProperties { game_id: 2, id: 6, name: "The Arbiter".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Self::TheOracle => MissionProperties { game_id: 2, id: 7, name: "The Oracle".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(16_000) },
            Self::DeltaHalo => MissionProperties { game_id: 2, id: 8, name: "Delta Halo".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(10_000) },
            Self::Regret => MissionProperties { game_id: 2, id: 9, name: "Regret".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
            Self::SacredIcon => MissionProperties { game_id: 2, id: 10, name: "Sacred Icon".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Self::QuarantineZone => MissionProperties { game_id: 2, id: 11, name: "Quarantine Zone".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(7_000) },
            Self::Gravemind => MissionProperties { game_id: 2, id: 12, name: "Gravemind".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(11_000) },
            Self::Uprising => MissionProperties { game_id: 2, id: 13, name: "Uprising".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Self::HighCharity => MissionProperties { game_id: 2, id: 14, name: "High Charity".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(9_000) },
            Self::TheGreatJourney => MissionProperties { game_id: 2, id: 15, name: "The Great Journey".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(8_000) },
        }
    }
}

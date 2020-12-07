use strum_macros::EnumIter;
use time::time;

use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
pub enum HaloReachMission {
    NobleActual,
    WinterContingency,
    ONISwordBase,
    Nightfall,
    TipOfTheSpear,
    LongNightOfSolace,
    Exodus,
    NewAlexandria,
    ThePackage,
    ThePillarOfAutumn,
    Epilogue,
    LoneWolf,
}

impl HaloReachMission {
    pub fn to_mission(&self) -> Mission {
        Mission::HaloReach(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::NobleActual => MissionProperties { game_id: 5, id: 1, name: "Noble Actual".to_string(), par_time: None, par_score: None },
            Self::WinterContingency => MissionProperties { game_id: 5, id: 2, name: "Winter Contingency".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(15_000) },
            Self::ONISwordBase => MissionProperties { game_id: 5, id: 3, name: "ONI Sword Base".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(25_000) },
            Self::Nightfall => MissionProperties { game_id: 5, id: 4, name: "Nightfall".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(7_500) },
            Self::TipOfTheSpear => MissionProperties { game_id: 5, id: 5, name: "Tip of The Spear".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(30_000) },
            Self::LongNightOfSolace => MissionProperties { game_id: 5, id: 6, name: "Long Night of Solace".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(45_000) },
            Self::Exodus => MissionProperties { game_id: 5, id: 7, name: "Exodus".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(30_000) },
            Self::NewAlexandria => MissionProperties { game_id: 5, id: 8, name: "New Alexandria".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(22_500) },
            Self::ThePackage => MissionProperties { game_id: 5, id: 9, name: "The Package".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(65_000) },
            Self::ThePillarOfAutumn => MissionProperties { game_id: 5, id: 10, name: "The Pillar of Autumn".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::Epilogue => MissionProperties { game_id: 5, id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
            Self::LoneWolf => MissionProperties { game_id: 5, id: 12, name: "Lone Wolf".to_string(), par_time: None, par_score: None },
        }
    }
}

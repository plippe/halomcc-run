use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
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

impl From<HaloReachMission> for Mission {
    fn from(mission: HaloReachMission) -> Self {
        Self::HaloReach(mission)
    }
}

impl From<HaloReachMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: HaloReachMission) -> Self {
        match mission {
            HaloReachMission::NobleActual => MissionProperties { game_id: Game::HaloReach.id(), id: 1, name: "Noble Actual".to_string(), par_time: None, par_score: None },
            HaloReachMission::WinterContingency => MissionProperties { game_id: Game::HaloReach.id(), id: 2, name: "Winter Contingency".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(15_000) },
            HaloReachMission::ONISwordBase => MissionProperties { game_id: Game::HaloReach.id(), id: 3, name: "ONI Sword Base".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(25_000) },
            HaloReachMission::Nightfall => MissionProperties { game_id: Game::HaloReach.id(), id: 4, name: "Nightfall".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(7_500) },
            HaloReachMission::TipOfTheSpear => MissionProperties { game_id: Game::HaloReach.id(), id: 5, name: "Tip of The Spear".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(30_000) },
            HaloReachMission::LongNightOfSolace => MissionProperties { game_id: Game::HaloReach.id(), id: 6, name: "Long Night of Solace".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(45_000) },
            HaloReachMission::Exodus => MissionProperties { game_id: Game::HaloReach.id(), id: 7, name: "Exodus".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(30_000) },
            HaloReachMission::NewAlexandria => MissionProperties { game_id: Game::HaloReach.id(), id: 8, name: "New Alexandria".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(22_500) },
            HaloReachMission::ThePackage => MissionProperties { game_id: Game::HaloReach.id(), id: 9, name: "The Package".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(65_000) },
            HaloReachMission::ThePillarOfAutumn => MissionProperties { game_id: Game::HaloReach.id(), id: 10, name: "The Pillar of Autumn".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            HaloReachMission::Epilogue => MissionProperties { game_id: Game::HaloReach.id(), id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
            HaloReachMission::LoneWolf => MissionProperties { game_id: Game::HaloReach.id(), id: 12, name: "Lone Wolf".to_string(), par_time: None, par_score: None },
        }
    }
}

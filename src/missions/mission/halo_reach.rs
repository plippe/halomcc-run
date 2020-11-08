use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
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

impl From<&HaloReachMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &HaloReachMission) -> Self {
        match mission {
            HaloReachMission::NobleActual => MissionProperties::new(Game::HaloReach, 1, "Noble Actual".to_string(), None, None),
            HaloReachMission::WinterContingency => MissionProperties::new(Game::HaloReach, 2, "Winter Contingency".to_string(), Some(time!(00:15:00)), Some(15_000)),
            HaloReachMission::ONISwordBase => MissionProperties::new(Game::HaloReach, 3, "ONI Sword Base".to_string(), Some(time!(00:10:00)), Some(25_000)),
            HaloReachMission::Nightfall => MissionProperties::new(Game::HaloReach, 4, "Nightfall".to_string(), Some(time!(00:10:00)), Some(7_500)),
            HaloReachMission::TipOfTheSpear => MissionProperties::new(Game::HaloReach, 5, "Tip of The Spear".to_string(), Some(time!(00:15:00)), Some(30_000)),
            HaloReachMission::LongNightOfSolace => MissionProperties::new(Game::HaloReach, 6, "Long Night of Solace".to_string(), Some(time!(00:25:00)), Some(45_000)),
            HaloReachMission::Exodus => MissionProperties::new(Game::HaloReach, 7, "Exodus".to_string(), Some(time!(00:20:00)), Some(30_000)),
            HaloReachMission::NewAlexandria => MissionProperties::new(Game::HaloReach, 8, "New Alexandria".to_string(), Some(time!(00:20:00)), Some(22_500)),
            HaloReachMission::ThePackage => MissionProperties::new(Game::HaloReach, 9, "The Package".to_string(), Some(time!(00:20:00)), Some(65_000)),
            HaloReachMission::ThePillarOfAutumn => MissionProperties::new(Game::HaloReach, 10, "The Pillar of Autumn".to_string(), Some(time!(00:20:00)), Some(25_000)),
            HaloReachMission::Epilogue => MissionProperties::new(Game::HaloReach, 11, "Epilogue".to_string(), None, None),
            HaloReachMission::LoneWolf => MissionProperties::new(Game::HaloReach, 12, "Lone Wolf".to_string(), None, None),
        }
    }
}

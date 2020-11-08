use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
pub enum HaloMission {
    PillarOfAutumn,
    Halo,
    TruthAndReconciliation,
    SilentCartographer,
    AssaultOnTheControlRoom,
    ThreeFourThreeGuiltySpark,
    TheLibrary,
    TwoBetrayals,
    Keyes,
    TheMaw,
}

impl From<HaloMission> for Mission {
    fn from(mission: HaloMission) -> Self {
        Self::Halo(mission)
    }
}

impl From<&HaloMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &HaloMission) -> Self {
        match mission {
            HaloMission::PillarOfAutumn => MissionProperties::new(Game::Halo, 1, "Pillar of Autumn".to_string(), Some(time!(00:15:00)), Some(17_000)),
            HaloMission::Halo => MissionProperties::new(Game::Halo, 2, "Halo".to_string(), Some(time!(00:20:00)), Some(12_000)),
            HaloMission::TruthAndReconciliation => MissionProperties::new(Game::Halo, 3, "Truth and Reconciliation".to_string(), Some(time!(00:20:00)), Some(19_000)),
            HaloMission::SilentCartographer => MissionProperties::new(Game::Halo, 4, "Silent Cartographer".to_string(), Some(time!(00:15:00)), Some(18_000)),
            HaloMission::AssaultOnTheControlRoom => MissionProperties::new(Game::Halo, 5, "Assault on the Control Room".to_string(), Some(time!(00:15:00)), Some(18_000)),
            HaloMission::ThreeFourThreeGuiltySpark => MissionProperties::new(Game::Halo, 6, "343 Guilty Spark".to_string(), Some(time!(00:15:00)), Some(17_000)),
            HaloMission::TheLibrary => MissionProperties::new(Game::Halo, 7, "The Library".to_string(), Some(time!(00:25:00)), Some(25_000)),
            HaloMission::TwoBetrayals => MissionProperties::new(Game::Halo, 8, "Two Betrayals".to_string(), Some(time!(00:20:00)), Some(16_000)),
            HaloMission::Keyes => MissionProperties::new(Game::Halo, 9, "Keyes".to_string(), Some(time!(00:15:00)), Some(20_000)),
            HaloMission::TheMaw => MissionProperties::new(Game::Halo, 10, "The Maw".to_string(), Some(time!(00:15:00)), Some(18_000)),
        }
    }
}

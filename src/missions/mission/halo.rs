use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
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

impl From<HaloMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: HaloMission) -> Self {
        match mission {
            HaloMission::PillarOfAutumn => MissionProperties { game_id: Game::Halo.id(), id: 1, name: "Pillar of Autumn".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            HaloMission::Halo => MissionProperties { game_id: Game::Halo.id(), id: 2, name: "Halo".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(12_000) },
            HaloMission::TruthAndReconciliation => MissionProperties { game_id: Game::Halo.id(), id: 3, name: "Truth and Reconciliation".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(19_000) },
            HaloMission::SilentCartographer => MissionProperties { game_id: Game::Halo.id(), id: 4, name: "Silent Cartographer".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
            HaloMission::AssaultOnTheControlRoom => MissionProperties { game_id: Game::Halo.id(), id: 5, name: "Assault on the Control Room".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
            HaloMission::ThreeFourThreeGuiltySpark => MissionProperties { game_id: Game::Halo.id(), id: 6, name: "343 Guilty Spark".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            HaloMission::TheLibrary => MissionProperties { game_id: Game::Halo.id(), id: 7, name: "The Library".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            HaloMission::TwoBetrayals => MissionProperties { game_id: Game::Halo.id(), id: 8, name: "Two Betrayals".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(16_000) },
            HaloMission::Keyes => MissionProperties { game_id: Game::Halo.id(), id: 9, name: "Keyes".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(20_000) },
            HaloMission::TheMaw => MissionProperties { game_id: Game::Halo.id(), id: 10, name: "The Maw".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
        }
    }
}

use strum_macros::EnumIter;
use time::time;

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

impl HaloMission {
    pub fn to_mission(&self) -> Mission {
        Mission::Halo(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::PillarOfAutumn => MissionProperties { game_id: 1, id: 1, name: "Pillar of Autumn".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            Self::Halo => MissionProperties { game_id: 1, id: 2, name: "Halo".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(12_000) },
            Self::TruthAndReconciliation => MissionProperties { game_id: 1, id: 3, name: "Truth and Reconciliation".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(19_000) },
            Self::SilentCartographer => MissionProperties { game_id: 1, id: 4, name: "Silent Cartographer".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
            Self::AssaultOnTheControlRoom => MissionProperties { game_id: 1, id: 5, name: "Assault on the Control Room".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
            Self::ThreeFourThreeGuiltySpark => MissionProperties { game_id: 1, id: 6, name: "343 Guilty Spark".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(17_000) },
            Self::TheLibrary => MissionProperties { game_id: 1, id: 7, name: "The Library".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            Self::TwoBetrayals => MissionProperties { game_id: 1, id: 8, name: "Two Betrayals".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(16_000) },
            Self::Keyes => MissionProperties { game_id: 1, id: 9, name: "Keyes".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(20_000) },
            Self::TheMaw => MissionProperties { game_id: 1, id: 10, name: "The Maw".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(18_000) },
        }
    }
}

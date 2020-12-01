use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
pub enum Halo3OdstMission {
    PrepareToDrop,
    MombasaStreets,
    TayariPlaza,
    UpliftReserve,
    KizingoBoulevard,
    ONIAlphaSite,
    NMPDHQ,
    KikowaniStation,
    DataHive,
    CoastalHighway,
    Epilogue,
}

impl From<Halo3OdstMission> for Mission {
    fn from(mission: Halo3OdstMission) -> Self {
        Self::Halo3Odst(mission)
    }
}

impl From<Halo3OdstMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: Halo3OdstMission) -> Self {
        match mission {
            Halo3OdstMission::PrepareToDrop => MissionProperties { game_id: Game::Halo3Odst.id(), id: 1, name: "Prepare To Drop".to_string(), par_time: None, par_score: None },
            Halo3OdstMission::MombasaStreets => MissionProperties { game_id: Game::Halo3Odst.id(), id: 2, name: "Mombasa Streets".to_string(), par_time: None, par_score: None },
            Halo3OdstMission::TayariPlaza => MissionProperties { game_id: Game::Halo3Odst.id(), id: 3, name: "Tayari Plaza".to_string(), par_time: Some(time!(00:03:00)), par_score: Some(8_000) },
            Halo3OdstMission::UpliftReserve => MissionProperties { game_id: Game::Halo3Odst.id(), id: 4, name: "Uplift Reserve".to_string(), par_time: Some(time!(00:04:00)), par_score: Some(14_000) },
            Halo3OdstMission::KizingoBoulevard => MissionProperties { game_id: Game::Halo3Odst.id(), id: 5, name: "Kizingo Boulevard".to_string(), par_time: Some(time!(00:09:00)), par_score: Some(18_000) },
            Halo3OdstMission::ONIAlphaSite => MissionProperties { game_id: Game::Halo3Odst.id(), id: 6, name: "ONI Alpha Site".to_string(), par_time: Some(time!(00:13:00)), par_score: Some(16_000) },
            Halo3OdstMission::NMPDHQ => MissionProperties { game_id: Game::Halo3Odst.id(), id: 7, name: "NMPD HQ".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(40_000) },
            Halo3OdstMission::KikowaniStation => MissionProperties { game_id: Game::Halo3Odst.id(), id: 8, name: "Kikowani Station".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(42_000) },
            Halo3OdstMission::DataHive => MissionProperties { game_id: Game::Halo3Odst.id(), id: 9, name: "Data Hive".to_string(), par_time: Some(time!(00:16:00)), par_score: Some(8_000) },
            Halo3OdstMission::CoastalHighway => MissionProperties { game_id: Game::Halo3Odst.id(), id: 10, name: "Coastal Highway".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(90_000) },
            Halo3OdstMission::Epilogue => MissionProperties { game_id: Game::Halo3Odst.id(), id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

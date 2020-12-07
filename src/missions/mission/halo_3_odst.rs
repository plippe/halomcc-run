use strum_macros::EnumIter;
use time::time;

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

impl Halo3OdstMission {
    pub fn to_mission(&self) -> Mission {
        Mission::Halo3Odst(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::PrepareToDrop => MissionProperties { game_id: 4, id: 1, name: "Prepare To Drop".to_string(), par_time: None, par_score: None },
            Self::MombasaStreets => MissionProperties { game_id: 4, id: 2, name: "Mombasa Streets".to_string(), par_time: None, par_score: None },
            Self::TayariPlaza => MissionProperties { game_id: 4, id: 3, name: "Tayari Plaza".to_string(), par_time: Some(time!(00:03:00)), par_score: Some(8_000) },
            Self::UpliftReserve => MissionProperties { game_id: 4, id: 4, name: "Uplift Reserve".to_string(), par_time: Some(time!(00:04:00)), par_score: Some(14_000) },
            Self::KizingoBoulevard => MissionProperties { game_id: 4, id: 5, name: "Kizingo Boulevard".to_string(), par_time: Some(time!(00:09:00)), par_score: Some(18_000) },
            Self::ONIAlphaSite => MissionProperties { game_id: 4, id: 6, name: "ONI Alpha Site".to_string(), par_time: Some(time!(00:13:00)), par_score: Some(16_000) },
            Self::NMPDHQ => MissionProperties { game_id: 4, id: 7, name: "NMPD HQ".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(40_000) },
            Self::KikowaniStation => MissionProperties { game_id: 4, id: 8, name: "Kikowani Station".to_string(), par_time: Some(time!(00:10:00)), par_score: Some(42_000) },
            Self::DataHive => MissionProperties { game_id: 4, id: 9, name: "Data Hive".to_string(), par_time: Some(time!(00:16:00)), par_score: Some(8_000) },
            Self::CoastalHighway => MissionProperties { game_id: 4, id: 10, name: "Coastal Highway".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(90_000) },
            Self::Epilogue => MissionProperties { game_id: 4, id: 11, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
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

impl From<&Halo3OdstMission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &Halo3OdstMission) -> Self {
        match mission {
            Halo3OdstMission::PrepareToDrop => MissionProperties::new(Game::Halo3Odst, 1, "Prepare To Drop".to_string(), None, None),
            Halo3OdstMission::MombasaStreets => MissionProperties::new(Game::Halo3Odst, 2, "Mombasa Streets".to_string(), None, None),
            Halo3OdstMission::TayariPlaza => MissionProperties::new(Game::Halo3Odst, 3, "Tayari Plaza".to_string(), Some(time!(00:03:00)), Some(8_000)),
            Halo3OdstMission::UpliftReserve => MissionProperties::new(Game::Halo3Odst, 4, "Uplift Reserve".to_string(), Some(time!(00:04:00)), Some(14_000)),
            Halo3OdstMission::KizingoBoulevard => MissionProperties::new(Game::Halo3Odst, 5, "Kizingo Boulevard".to_string(), Some(time!(00:09:00)), Some(18_000)),
            Halo3OdstMission::ONIAlphaSite => MissionProperties::new(Game::Halo3Odst, 6, "ONI Alpha Site".to_string(), Some(time!(00:13:00)), Some(16_000)),
            Halo3OdstMission::NMPDHQ => MissionProperties::new(Game::Halo3Odst, 7, "NMPD HQ".to_string(), Some(time!(00:10:00)), Some(40_000)),
            Halo3OdstMission::KikowaniStation => MissionProperties::new(Game::Halo3Odst, 8, "Kikowani Station".to_string(), Some(time!(00:10:00)), Some(42_000)),
            Halo3OdstMission::DataHive => MissionProperties::new(Game::Halo3Odst, 9, "Data Hive".to_string(), Some(time!(00:16:00)), Some(8_000)),
            Halo3OdstMission::CoastalHighway => MissionProperties::new(Game::Halo3Odst, 10, "Coastal Highway".to_string(), Some(time!(00:25:00)), Some(90_000)),
            Halo3OdstMission::Epilogue => MissionProperties::new(Game::Halo3Odst, 11, "Epilogue".to_string(), None, None),
        }
    }
}

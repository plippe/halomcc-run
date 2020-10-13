use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone)]
pub enum Halo3OdstMission {
    TayariPlaza,
    UpliftReserve,
    KizingoBoulevard,
    ONIAlphaSite,
    NMPDHQ,
    KikowaniStation,
    DataHive,
    CoastalHighway,
}

impl Halo3OdstMission {
    pub fn all() -> Vec<Self> {
        vec![
            Halo3OdstMission::TayariPlaza,
            Halo3OdstMission::UpliftReserve,
            Halo3OdstMission::KizingoBoulevard,
            Halo3OdstMission::ONIAlphaSite,
            Halo3OdstMission::NMPDHQ,
            Halo3OdstMission::KikowaniStation,
            Halo3OdstMission::DataHive,
            Halo3OdstMission::CoastalHighway,
        ]
    }
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
            Halo3OdstMission::TayariPlaza => MissionProperties::new(Game::Halo3Odst, 1, "Tayari Plaza".to_string(), Some(time!(00:03:00))),
            Halo3OdstMission::UpliftReserve => MissionProperties::new(Game::Halo3Odst, 2, "Uplift Reserve".to_string(), Some(time!(00:04:00))),
            Halo3OdstMission::KizingoBoulevard => MissionProperties::new(Game::Halo3Odst, 3, "Kizingo Boulevard".to_string(), Some(time!(00:09:00))),
            Halo3OdstMission::ONIAlphaSite => MissionProperties::new(Game::Halo3Odst, 4, "ONI Alpha Site".to_string(), Some(time!(00:13:00))),
            Halo3OdstMission::NMPDHQ => MissionProperties::new(Game::Halo3Odst, 5, "NMPD HQ".to_string(), Some(time!(00:10:00))),
            Halo3OdstMission::KikowaniStation => MissionProperties::new(Game::Halo3Odst, 6, "Kikowani Station".to_string(), Some(time!(00:10:00))),
            Halo3OdstMission::DataHive => MissionProperties::new(Game::Halo3Odst, 7, "Data Hive".to_string(), Some(time!(00:16:00))),
            Halo3OdstMission::CoastalHighway => MissionProperties::new(Game::Halo3Odst, 8, "Coastal Highway".to_string(), Some(time!(00:25:00))),
        }
    }
}

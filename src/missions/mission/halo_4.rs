use strum_macros::EnumIter;
use time::time;

use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, Copy, EnumIter)]
pub enum Halo4Mission {
    Prologue,
    Dawn,
    Requiem,
    Forerunner,
    Infinity,
    Reclaimer,
    Shutdown,
    Composer,
    Midnight,
    Epilogue,
}

impl Halo4Mission {
    pub fn to_mission(&self) -> Mission {
        Mission::Halo4(*self)
    }

    #[rustfmt::skip]
    pub fn to_properties(&self) -> MissionProperties {
        match self {
            Self::Prologue => MissionProperties { game_id: 6, id: 1, name: "Prologue".to_string(), par_time: None, par_score: None },
            Self::Dawn => MissionProperties { game_id: 6, id: 2, name: "Dawn".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(25_000) },
            Self::Requiem => MissionProperties { game_id: 6, id: 3, name: "Requiem".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(22_000) },
            Self::Forerunner => MissionProperties { game_id: 6, id: 4, name: "Forerunner".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(22_000) },
            Self::Infinity => MissionProperties { game_id: 6, id: 5, name: "Infinity".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            Self::Reclaimer => MissionProperties { game_id: 6, id: 6, name: "Reclaimer".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::Shutdown => MissionProperties { game_id: 6, id: 7, name: "Shutdown".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::Composer => MissionProperties { game_id: 6, id: 8, name: "Composer".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Self::Midnight => MissionProperties { game_id: 6, id: 9, name: "Midnight".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            Self::Epilogue => MissionProperties { game_id: 6, id: 10, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

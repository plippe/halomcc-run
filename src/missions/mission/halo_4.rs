use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone, EnumIter)]
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

impl From<Halo4Mission> for Mission {
    fn from(mission: Halo4Mission) -> Self {
        Self::Halo4(mission)
    }
}

impl From<&Halo4Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: &Halo4Mission) -> Self {
        match mission {
            Halo4Mission::Prologue => MissionProperties::new(Game::Halo4, 1, "Prologue".to_string(), None, None),
            Halo4Mission::Dawn => MissionProperties::new(Game::Halo4, 2, "Dawn".to_string(), Some(time!(00:15:00)), Some(25_000)),
            Halo4Mission::Requiem => MissionProperties::new(Game::Halo4, 3, "Requiem".to_string(), Some(time!(00:15:00)), Some(22_000)),
            Halo4Mission::Forerunner => MissionProperties::new(Game::Halo4, 4, "Forerunner".to_string(), Some(time!(00:20:00)), Some(22_000)),
            Halo4Mission::Infinity => MissionProperties::new(Game::Halo4, 5, "Infinity".to_string(), Some(time!(00:25:00)), Some(25_000)),
            Halo4Mission::Reclaimer => MissionProperties::new(Game::Halo4, 6, "Reclaimer".to_string(), Some(time!(00:20:00)), Some(25_000)),
            Halo4Mission::Shutdown => MissionProperties::new(Game::Halo4, 7, "Shutdown".to_string(), Some(time!(00:20:00)), Some(25_000)),
            Halo4Mission::Composer => MissionProperties::new(Game::Halo4, 8, "Composer".to_string(), Some(time!(00:20:00)), Some(25_000)),
            Halo4Mission::Midnight => MissionProperties::new(Game::Halo4, 9, "Midnight".to_string(), Some(time!(00:25:00)), Some(25_000)),
            Halo4Mission::Epilogue => MissionProperties::new(Game::Halo4, 10, "Epilogue".to_string(), None, None),
        }
    }
}

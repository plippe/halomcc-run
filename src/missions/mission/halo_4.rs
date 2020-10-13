use time::time;

use crate::games::game::Game;
use crate::missions::mission::{Mission, MissionProperties};

#[derive(Clone)]
pub enum Halo4Mission {
    Dawn,
    Requiem,
    Forerunner,
    Infinity,
    Reclaimer,
    Shutdown,
    Composer,
    Midnight,
}

impl Halo4Mission {
    pub fn all() -> Vec<Self> {
        vec![
            Halo4Mission::Dawn,
            Halo4Mission::Requiem,
            Halo4Mission::Forerunner,
            Halo4Mission::Infinity,
            Halo4Mission::Reclaimer,
            Halo4Mission::Shutdown,
            Halo4Mission::Composer,
            Halo4Mission::Midnight,
        ]
    }
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
            Halo4Mission::Dawn => MissionProperties::new(Game::Halo4, 1, "Dawn".to_string(), Some(time!(00:15:00))),
            Halo4Mission::Requiem => MissionProperties::new(Game::Halo4, 2, "Requiem".to_string(), Some(time!(00:15:00))),
            Halo4Mission::Forerunner => MissionProperties::new(Game::Halo4, 3, "Forerunner".to_string(), Some(time!(00:20:00))),
            Halo4Mission::Infinity => MissionProperties::new(Game::Halo4, 4, "Infinity".to_string(), Some(time!(00:25:00))),
            Halo4Mission::Reclaimer => MissionProperties::new(Game::Halo4, 5, "Reclaimer".to_string(), Some(time!(00:20:00))),
            Halo4Mission::Shutdown => MissionProperties::new(Game::Halo4, 6, "Shutdown".to_string(), Some(time!(00:20:00))),
            Halo4Mission::Composer => MissionProperties::new(Game::Halo4, 7, "Composer".to_string(), Some(time!(00:20:00))),
            Halo4Mission::Midnight => MissionProperties::new(Game::Halo4, 8, "Midnight".to_string(), Some(time!(00:25:00))),
        }
    }
}

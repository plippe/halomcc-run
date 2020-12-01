use strum_macros::EnumIter;
use time::time;

use crate::games::game::Game;
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

impl From<Halo4Mission> for Mission {
    fn from(mission: Halo4Mission) -> Self {
        Self::Halo4(mission)
    }
}

impl From<Halo4Mission> for MissionProperties {
    #[rustfmt::skip]
    fn from(mission: Halo4Mission) -> Self {
        match mission {
            Halo4Mission::Prologue => MissionProperties { game_id: Game::Halo4.id(), id: 1, name: "Prologue".to_string(), par_time: None, par_score: None },
            Halo4Mission::Dawn => MissionProperties { game_id: Game::Halo4.id(), id: 2, name: "Dawn".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(25_000) },
            Halo4Mission::Requiem => MissionProperties { game_id: Game::Halo4.id(), id: 3, name: "Requiem".to_string(), par_time: Some(time!(00:15:00)), par_score: Some(22_000) },
            Halo4Mission::Forerunner => MissionProperties { game_id: Game::Halo4.id(), id: 4, name: "Forerunner".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(22_000) },
            Halo4Mission::Infinity => MissionProperties { game_id: Game::Halo4.id(), id: 5, name: "Infinity".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            Halo4Mission::Reclaimer => MissionProperties { game_id: Game::Halo4.id(), id: 6, name: "Reclaimer".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Halo4Mission::Shutdown => MissionProperties { game_id: Game::Halo4.id(), id: 7, name: "Shutdown".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Halo4Mission::Composer => MissionProperties { game_id: Game::Halo4.id(), id: 8, name: "Composer".to_string(), par_time: Some(time!(00:20:00)), par_score: Some(25_000) },
            Halo4Mission::Midnight => MissionProperties { game_id: Game::Halo4.id(), id: 9, name: "Midnight".to_string(), par_time: Some(time!(00:25:00)), par_score: Some(25_000) },
            Halo4Mission::Epilogue => MissionProperties { game_id: Game::Halo4.id(), id: 10, name: "Epilogue".to_string(), par_time: None, par_score: None },
        }
    }
}

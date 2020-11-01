use std::result::Result;
use std::str::FromStr;

use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game;

impl Game {
    pub fn missions_id_delta(self) -> i32 {
        match self {
            Game::Halo => 1,
            Game::Halo2 => -30,
            Game::Halo3 => -69,
            Game::Halo3Odst => -167,
            Game::HaloReach => -177,
            Game::Halo4 => -103,
        }
    }
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        match game {
            "HaloCombatEvolved" => Ok(Game::Halo),
            "Halo2" => Ok(Game::Halo2),
            "Halo3" => Ok(Game::Halo3),
            "Halo3Odst" => Ok(Game::Halo3Odst),
            "HaloReach" => Ok(Game::HaloReach),
            "Halo4" => Ok(Game::Halo4),
            game => Err(HaloWaypointError::UnknownGame {
                game: game.to_string(),
            }
            .into()),
        }
    }
}

impl ToString for Game {
    fn to_string(&self) -> String {
        match self {
            Game::Halo => "HaloCombatEvolved".to_string(),
            Game::Halo2 => "Halo2".to_string(),
            Game::Halo3 => "Halo3".to_string(),
            Game::Halo3Odst => "Halo3Odst".to_string(),
            Game::HaloReach => "HaloReach".to_string(),
            Game::Halo4 => "Halo4".to_string(),
        }
    }
}

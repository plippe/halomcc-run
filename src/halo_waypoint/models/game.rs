use std::result::Result;
use std::str::FromStr;

use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game as MyGame;

#[derive(Debug, PartialEq)]
pub enum Game {
    HaloReach,
    HaloCombatEvolved,
    Halo2,
    Halo3,
    Halo3Odst,
    Halo4,
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        match game {
            "HaloReach" => Ok(Game::HaloReach),
            "HaloCombatEvolved" => Ok(Game::HaloCombatEvolved),
            "Halo2" => Ok(Game::Halo2),
            "Halo3" => Ok(Game::Halo3),
            "Halo3Odst" => Ok(Game::Halo3Odst),
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
            Game::HaloReach => "HaloReach".to_string(),
            Game::HaloCombatEvolved => "HaloCombatEvolved".to_string(),
            Game::Halo2 => "Halo2".to_string(),
            Game::Halo3 => "Halo3".to_string(),
            Game::Halo3Odst => "Halo3Odst".to_string(),
            Game::Halo4 => "Halo4".to_string(),
        }
    }
}

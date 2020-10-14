use std::result::Result;
use std::str::FromStr;

use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game as MyGame;

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl From<&MyGame> for Game {
    fn from(game: &MyGame) -> Self {
        match game {
            MyGame::Halo => Game::HaloCombatEvolved,
            MyGame::Halo2 => Game::Halo2,
            MyGame::Halo3 => Game::Halo3,
            MyGame::Halo3Odst => Game::Halo3Odst,
            MyGame::HaloReach => Game::HaloReach,
            MyGame::Halo4 => Game::Halo4,
        }
    }
}

impl Into<MyGame> for Game {
    fn into(self) -> MyGame {
        match self {
            Game::HaloCombatEvolved => MyGame::Halo,
            Game::Halo2 => MyGame::Halo2,
            Game::Halo3 => MyGame::Halo3,
            Game::Halo3Odst => MyGame::Halo3Odst,
            Game::HaloReach => MyGame::HaloReach,
            Game::Halo4 => MyGame::Halo4,
        }
    }
}

use std::result::Result;
use std::str::FromStr;

use crate::error::Error;

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
    fn from_str(it: &str) -> Result<Self, Self::Err> {
        match it {
            "HaloReach" => Ok(Game::HaloReach),
            "HaloCombatEvolved" => Ok(Game::HaloCombatEvolved),
            "Halo2" => Ok(Game::Halo2),
            "Halo3" => Ok(Game::Halo3),
            "Halo3Odst" => Ok(Game::Halo3Odst),
            "Halo4" => Ok(Game::Halo4),
            it => Err(Error::HaloWaypointUnknownGame {
                game: it.to_string(),
            }),
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

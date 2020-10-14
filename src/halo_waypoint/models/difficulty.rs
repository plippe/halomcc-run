use std::result::Result;
use std::str::FromStr;

use crate::error::{Error, HaloWaypointError};

#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Heroic,
    Legendary,
}

impl FromStr for Difficulty {
    type Err = Error;
    fn from_str(difficulty: &str) -> Result<Self, Self::Err> {
        match difficulty {
            "Easy" => Ok(Difficulty::Easy),
            "Normal" => Ok(Difficulty::Normal),
            "Heroic" => Ok(Difficulty::Heroic),
            "Legendary" => Ok(Difficulty::Legendary),
            difficulty => Err(HaloWaypointError::UnknownDifficulty {
                difficulty: difficulty.to_string(),
            }
            .into()),
        }
    }
}

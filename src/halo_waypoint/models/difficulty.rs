use std::result::Result;
use std::str::FromStr;

use crate::difficulties::difficulty::Difficulty;
use crate::error::{Error, HaloWaypointError};

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

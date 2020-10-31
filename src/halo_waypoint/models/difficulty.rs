use std::result::Result;
use std::str::FromStr;

use crate::difficulties::difficulty::Difficulty as MyDifficulty;
use crate::error::{Error, HaloWaypointError};

#[derive(Clone, Copy, Debug, PartialEq)]
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

impl Into<MyDifficulty> for Difficulty {
    fn into(self) -> MyDifficulty {
        match self {
            Difficulty::Easy => MyDifficulty::Easy,
            Difficulty::Normal => MyDifficulty::Normal,
            Difficulty::Heroic => MyDifficulty::Heroic,
            Difficulty::Legendary => MyDifficulty::Legendary,
        }
    }
}

use std::result::Result;
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum Difficulty {
    Easy,
    Normal,
    Heroic,
    Legendary,
}

impl FromStr for Difficulty {
    type Err = Error;
    fn from_str(it: &str) -> Result<Self, Self::Err> {
        match it {
            "Easy" => Ok(Difficulty::Easy),
            "Normal" => Ok(Difficulty::Normal),
            "Heroic" => Ok(Difficulty::Heroic),
            "Legendary" => Ok(Difficulty::Legendary),
            it => Err(Error::HaloWaypointUnknownDifficulty {
                difficulty: it.to_string(),
            }),
        }
    }
}

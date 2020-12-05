use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;

use crate::difficulties::difficulty::Difficulty as InternalDifficulty;
use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Difficulty {
    None,
    Easy,
    Normal,
    Heroic,
    Legendary,
}

impl Difficulty {
    const NONE: &'static str = "None";
    const EASY: &'static str = "Easy";
    const NORMAL: &'static str = "Normal";
    const HEROIC: &'static str = "Heroic";
    const LEGENDARY: &'static str = "Legendary";
}

impl FromStr for Difficulty {
    type Err = Error;
    fn from_str(difficulty: &str) -> Result<Self, Self::Err> {
        match difficulty {
            Self::NONE => Ok(Self::None),
            Self::EASY => Ok(Self::Easy),
            Self::NORMAL => Ok(Self::Normal),
            Self::HEROIC => Ok(Self::Heroic),
            Self::LEGENDARY => Ok(Self::Legendary),
            difficulty => Err(HaloWaypointError::UnknownDifficulty {
                difficulty: difficulty.to_string(),
            }
            .into()),
        }
    }
}

impl<'a> TryFrom<ElementRef<'a>> for Difficulty {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let selector = Selector::parse(".skull .spritesheet").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("title"))
            .ok_or_else(|| HaloWaypointError::MissingDifficulty.into())
            .and_then(Self::from_str)
    }
}

impl From<&Difficulty> for Option<InternalDifficulty> {
    fn from(difficulty: &Difficulty) -> Self {
        match difficulty {
            Difficulty::None => None,
            Difficulty::Easy => Some(InternalDifficulty::Easy),
            Difficulty::Normal => Some(InternalDifficulty::Normal),
            Difficulty::Heroic => Some(InternalDifficulty::Heroic),
            Difficulty::Legendary => Some(InternalDifficulty::Legendary),
        }
    }
}

use scraper::{ElementRef, Selector};
use std::result::Result;

use crate::chainable::Chainable;
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

    fn try_from_str(difficulty: &str) -> Result<Self, Error> {
        match difficulty {
            Self::NONE => Ok(Self::None),
            Self::EASY => Ok(Self::Easy),
            Self::NORMAL => Ok(Self::Normal),
            Self::HEROIC => Ok(Self::Heroic),
            Self::LEGENDARY => Ok(Self::Legendary),
            difficulty => HaloWaypointError::UnknownDifficulty(difficulty.to_string())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }

    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(".skull .spritesheet").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("title"))
            .ok_or_else(|| HaloWaypointError::MissingDifficulty.pipe(Error::HaloWaypoint))
            .and_then(Self::try_from_str)
    }

    pub fn to_internal(&self) -> Option<InternalDifficulty> {
        match self {
            Self::None => None,
            Self::Easy => Some(InternalDifficulty::Easy),
            Self::Normal => Some(InternalDifficulty::Normal),
            Self::Heroic => Some(InternalDifficulty::Heroic),
            Self::Legendary => Some(InternalDifficulty::Legendary),
        }
    }
}

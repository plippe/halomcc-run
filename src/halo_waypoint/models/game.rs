use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;

use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game as InternalGame;

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Game {
    Halo,
    Halo2,
    Halo3,
    Halo3Odst,
    HaloReach,
    Halo4,
}

impl Game {
    const HALO: &'static str = "HaloCombatEvolved";
    const HALO_2: &'static str = "Halo2";
    const HALO_3: &'static str = "Halo3";
    const HALO_3_ODST: &'static str = "Halo3Odst";
    const HALO_REACH: &'static str = "HaloReach";
    const HALO_4: &'static str = "Halo4";
}

impl FromStr for Game {
    type Err = Error;
    fn from_str(game: &str) -> Result<Self, Self::Err> {
        match game {
            Self::HALO => Ok(Self::Halo),
            Self::HALO_2 => Ok(Self::Halo2),
            Self::HALO_3 => Ok(Self::Halo3),
            Self::HALO_3_ODST => Ok(Self::Halo3Odst),
            Self::HALO_REACH => Ok(Self::HaloReach),
            Self::HALO_4 => Ok(Self::Halo4),
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
            Self::Halo => Self::HALO,
            Self::Halo2 => Self::HALO_2,
            Self::Halo3 => Self::HALO_3,
            Self::Halo3Odst => Self::HALO_3_ODST,
            Self::HaloReach => Self::HALO_REACH,
            Self::Halo4 => Self::HALO_4,
        }
        .to_string()
    }
}

impl<'a> TryFrom<ElementRef<'a>> for Game {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let selector = Selector::parse("[data-game-id]").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("data-game-id"))
            .ok_or_else(|| HaloWaypointError::MissingGame.into())
            .and_then(Self::from_str)
    }
}

// Transition code
impl Game {
    pub fn missions_id_delta(self) -> i32 {
        match self {
            Game::Halo => 1,
            Game::Halo2 => -28,
            Game::Halo3 => -68,
            Game::Halo3Odst => -165,
            Game::HaloReach => -177,
            Game::Halo4 => -102,
        }
    }
}

// Transition code
impl From<&InternalGame> for Game {
    fn from(game: &InternalGame) -> Self {
        match game {
            InternalGame::Halo => Self::Halo,
            InternalGame::Halo2 => Self::Halo2,
            InternalGame::Halo3 => Self::Halo3,
            InternalGame::Halo3Odst => Self::Halo3Odst,
            InternalGame::HaloReach => Self::HaloReach,
            InternalGame::Halo4 => Self::Halo4,
        }
    }
}

// Transition code
impl From<&Game> for InternalGame {
    fn from(game: &Game) -> Self {
        match game {
            Game::Halo => Self::Halo,
            Game::Halo2 => Self::Halo2,
            Game::Halo3 => Self::Halo3,
            Game::Halo3Odst => Self::Halo3Odst,
            Game::HaloReach => Self::HaloReach,
            Game::Halo4 => Self::Halo4,
        }
    }
}

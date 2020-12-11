use scraper::{ElementRef, Selector};
use std::result::Result;

use crate::chainable::Chainable;
use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game as InternalGame;
use crate::games::game::GameId as InternalGameId;

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

    pub fn try_from_str(game: &str) -> Result<Self, Error> {
        match game {
            Self::HALO => Ok(Self::Halo),
            Self::HALO_2 => Ok(Self::Halo2),
            Self::HALO_3 => Ok(Self::Halo3),
            Self::HALO_3_ODST => Ok(Self::Halo3Odst),
            Self::HALO_REACH => Ok(Self::HaloReach),
            Self::HALO_4 => Ok(Self::Halo4),
            game => HaloWaypointError::UnknownGame(game.to_string())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
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

    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse("[data-game-id]").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("data-game-id"))
            .ok_or_else(|| HaloWaypointError::MissingGame.pipe(Error::HaloWaypoint))
            .and_then(Self::try_from_str)
    }

    pub fn from_internal(game: &InternalGame) -> Self {
        match game.id().value() {
            1 => Self::Halo,
            2 => Self::Halo2,
            3 => Self::Halo3,
            4 => Self::Halo3Odst,
            5 => Self::HaloReach,
            6 => Self::Halo4,
            other => unreachable!("Internal game id shouldn't exist: {}", other),
        }
    }

    pub fn to_internal(&self) -> InternalGameId {
        let id = match self {
            Self::Halo => 1,
            Self::Halo2 => 2,
            Self::Halo3 => 3,
            Self::Halo3Odst => 4,
            Self::HaloReach => 5,
            Self::Halo4 => 6,
        };

        InternalGameId::new(id)
    }
}

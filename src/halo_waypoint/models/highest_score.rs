use scraper::{ElementRef, Selector};
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum HighestScore {
    Some(i32),
    None,
}

impl HighestScore {
    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(".highest-score").unwrap();

        element
            .select(&selector)
            .next()
            .ok_or(HaloWaypointError::MissingScore)
            .and_then(|element| match element.inner_html().as_str() {
                "--" => Ok(Self::None),
                html => html
                    .parse()
                    .map(Self::Some)
                    .map_err(|_| HaloWaypointError::InvalidScore(html.to_string())),
            })
            .map_err(Error::HaloWaypoint)
    }

    pub fn to_internal(&self) -> Option<i32> {
        match self {
            Self::Some(highest_score) => Some(*highest_score),
            Self::None => None,
        }
    }
}

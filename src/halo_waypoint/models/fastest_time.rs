use scraper::{ElementRef, Selector};
use std::result::Result;
use time::Time;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum FastestTime {
    Some(Time),
    None,
}

impl FastestTime {
    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse(".best-time").unwrap();

        element
            .select(&selector)
            .next()
            .ok_or(HaloWaypointError::MissingTime)
            .and_then(|element| match element.inner_html().as_str() {
                "--" => Ok(Self::None),
                html => Time::parse(html, "%T")
                    .map(Self::Some)
                    .map_err(|_| HaloWaypointError::InvalidTime(html.to_string())),
            })
            .map_err(Error::HaloWaypoint)
    }

    pub fn to_internal(&self) -> Option<Time> {
        match self {
            Self::Some(time) => Some(*time),
            Self::None => None,
        }
    }
}

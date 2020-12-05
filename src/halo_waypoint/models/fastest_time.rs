use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;
use time::Time;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum FastestTime {
    Some(Time),
    None,
}

impl<'a> TryFrom<ElementRef<'a>> for FastestTime {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let selector = Selector::parse(".best-time").unwrap();

        element
            .select(&selector)
            .next()
            .ok_or(HaloWaypointError::MissingTime)
            .and_then(|element| match element.inner_html().as_str() {
                "--" => Ok(FastestTime::None),
                html => Time::parse(html, "%T").map(FastestTime::Some).map_err(|_| {
                    HaloWaypointError::InvalidTime {
                        time: html.to_string(),
                    }
                }),
            })
            .map_err(|err| err.into())
    }
}

impl From<&FastestTime> for Option<Time> {
    fn from(fastest_time: &FastestTime) -> Self {
        match fastest_time {
            FastestTime::Some(time) => Some(*time),
            FastestTime::None => None,
        }
    }
}

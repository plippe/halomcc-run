use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;
use time::Time;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct FastestTime(Option<Time>);

impl FastestTime {
    pub fn new(fastest_time: Time) -> FastestTime {
        FastestTime(Some(fastest_time))
    }

    pub fn empty() -> FastestTime {
        FastestTime(None)
    }

    pub fn value(&self) -> Option<Time> {
        self.0
    }
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
                "--" => Ok(FastestTime::empty()),
                html => Time::parse(html, "%T").map(FastestTime::new).map_err(|_| {
                    HaloWaypointError::InvalidTime {
                        time: html.to_string(),
                    }
                }),
            })
            .map_err(|err| err.into())
    }
}

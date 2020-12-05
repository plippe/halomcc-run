use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct HighestScore(Option<i32>);

impl HighestScore {
    pub fn new(highest_score: i32) -> HighestScore {
        HighestScore(Some(highest_score))
    }

    pub fn empty() -> HighestScore {
        HighestScore(None)
    }

    pub fn value(&self) -> Option<i32> {
        self.0
    }
}

impl<'a> TryFrom<ElementRef<'a>> for HighestScore {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let selector = Selector::parse(".highest-score").unwrap();

        element
            .select(&selector)
            .next()
            .ok_or(HaloWaypointError::MissingScore)
            .and_then(|element| match element.inner_html().as_str() {
                "--" => Ok(HighestScore::empty()),
                html => html.parse().map(HighestScore::new).map_err(|_| {
                    HaloWaypointError::InvalidScore {
                        score: html.to_string(),
                    }
                }),
            })
            .map_err(|err| err.into())
    }
}

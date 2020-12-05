use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum HighestScore {
    Some(i32),
    None,
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
                "--" => Ok(HighestScore::None),
                html => html.parse().map(HighestScore::Some).map_err(|_| {
                    HaloWaypointError::InvalidScore {
                        score: html.to_string(),
                    }
                }),
            })
            .map_err(|err| err.into())
    }
}

impl From<&HighestScore> for Option<i32> {
    fn from(highest_score: &HighestScore) -> Self {
        match highest_score {
            HighestScore::Some(highest_score) => Some(*highest_score),
            HighestScore::None => None,
        }
    }
}

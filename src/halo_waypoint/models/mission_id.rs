use scraper::ElementRef;
use std::convert::TryFrom;
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MissionId(i32);

impl MissionId {
    pub fn new(mission: i32) -> Self {
        Self(mission)
    }

    pub fn value(&self) -> i32 {
        self.0
    }
}

impl<'a> TryFrom<ElementRef<'a>> for MissionId {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        element
            .value()
            .attr("data-mission-id")
            .ok_or(HaloWaypointError::MissingMissionId)
            .and_then(|attribute| {
                attribute
                    .parse()
                    .map_err(|_| HaloWaypointError::InvalidMissionId {
                        mission_id: attribute.to_string(),
                    })
            })
            .map(Self::new)
            .map_err(|err| err.into())
    }
}

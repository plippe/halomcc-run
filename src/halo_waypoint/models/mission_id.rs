use scraper::ElementRef;
use std::result::Result;

use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub struct MissionId(i32);

impl MissionId {
    pub fn new(mission: i32) -> Self {
        Self(mission)
    }

    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        element
            .value()
            .attr("data-mission-id")
            .ok_or(HaloWaypointError::MissingMissionId)
            .and_then(|attribute| {
                attribute
                    .parse()
                    .map_err(|_| HaloWaypointError::InvalidMissionId(attribute.to_string()))
            })
            .map(Self::new)
            .map_err(Error::HaloWaypoint)
    }

    pub fn to_internal(&self) -> i32 {
        match self.0 {
            0..=9 => self.0 + 1,
            31..=43 => self.0 - 28,
            70..=78 => self.0 - 68,
            168..=175 => self.0 - 165,
            178..=189 => self.0 - 177,
            104..=111 => self.0 - 102,
            other => unreachable!("External mission id shouldn't exist: {}", other),
        }
    }
}

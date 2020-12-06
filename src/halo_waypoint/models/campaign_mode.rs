use scraper::{ElementRef, Selector};
use std::result::Result;

use crate::campaign_modes::campaign_mode::CampaignMode as InternalCampaignMode;
use crate::chainable::Chainable;
use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CampaignMode {
    Solo,
    Coop,
}

impl CampaignMode {
    const SOLO: &'static str = "Solo";
    const COOP: &'static str = "Coop";

    fn try_from_str(campaign_mode: &str) -> Result<Self, Error> {
        match campaign_mode {
            Self::SOLO => Ok(Self::Solo),
            Self::COOP => Ok(Self::Coop),
            campaign_mode => HaloWaypointError::UnknownCampaignMode(campaign_mode.to_string())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }

    pub fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let selector = Selector::parse("[data-mode-id]").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("data-mode-id"))
            .ok_or_else(|| HaloWaypointError::MissingCampaignMode.pipe(Error::HaloWaypoint))
            .and_then(Self::try_from_str)
    }

    #[allow(clippy::inherent_to_string)]
    pub fn to_string(&self) -> String {
        match self {
            Self::Solo => Self::SOLO,
            Self::Coop => Self::COOP,
        }
        .to_string()
    }

    pub fn from_internal(campaign_mode: &InternalCampaignMode) -> Self {
        match campaign_mode {
            InternalCampaignMode::Solo => Self::Solo,
            InternalCampaignMode::Coop => Self::Coop,
        }
    }

    pub fn to_internal(&self) -> InternalCampaignMode {
        match self {
            Self::Solo => InternalCampaignMode::Solo,
            Self::Coop => InternalCampaignMode::Coop,
        }
    }
}

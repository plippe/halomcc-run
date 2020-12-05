use scraper::{ElementRef, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;
use std::string::ToString;

use crate::campaign_modes::campaign_mode::CampaignMode as InternalCampaignMode;
use crate::error::{Error, HaloWaypointError};

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum CampaignMode {
    Solo,
    Coop,
}

impl CampaignMode {
    const SOLO: &'static str = "Solo";
    const COOP: &'static str = "Coop";
}

impl FromStr for CampaignMode {
    type Err = Error;
    fn from_str(campaign_mode: &str) -> Result<Self, Self::Err> {
        match campaign_mode {
            Self::SOLO => Ok(Self::Solo),
            Self::COOP => Ok(Self::Coop),
            campaign_mode => Err(HaloWaypointError::UnknownCampaignMode {
                campaign_mode: campaign_mode.to_string(),
            }
            .into()),
        }
    }
}

impl ToString for CampaignMode {
    fn to_string(&self) -> String {
        match self {
            Self::Solo => Self::SOLO,
            Self::Coop => Self::COOP,
        }
        .to_string()
    }
}

impl<'a> TryFrom<ElementRef<'a>> for CampaignMode {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let selector = Selector::parse("[data-mode-id]").unwrap();

        element
            .select(&selector)
            .next()
            .and_then(|element| element.value().attr("data-mode-id"))
            .ok_or_else(|| HaloWaypointError::MissingCampaignMode.into())
            .and_then(Self::from_str)
    }
}

impl From<&CampaignMode> for InternalCampaignMode {
    fn from(campaign_mode: &CampaignMode) -> Self {
        match campaign_mode {
            CampaignMode::Solo => Self::Solo,
            CampaignMode::Coop => Self::Coop,
        }
    }
}

impl From<&InternalCampaignMode> for CampaignMode {
    fn from(campaign_mode: &InternalCampaignMode) -> Self {
        match campaign_mode {
            InternalCampaignMode::Solo => Self::Solo,
            InternalCampaignMode::Coop => Self::Coop,
        }
    }
}

use std::result::Result;
use std::str::FromStr;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::error::{Error, HaloWaypointError};

impl FromStr for CampaignMode {
    type Err = Error;
    fn from_str(campaign_mode: &str) -> Result<Self, Self::Err> {
        match campaign_mode {
            "Solo" => Ok(CampaignMode::Solo),
            "Coop" => Ok(CampaignMode::Coop),
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
            CampaignMode::Solo => "Solo".to_string(),
            CampaignMode::Coop => "Coop".to_string(),
        }
    }
}

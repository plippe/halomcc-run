use std::result::Result;
use std::str::FromStr;

use crate::error::Error;

#[derive(Debug, PartialEq)]
pub enum CampaignMode {
    Solo,
    Coop,
}

impl FromStr for CampaignMode {
    type Err = Error;
    fn from_str(campaign_mode: &str) -> Result<Self, Self::Err> {
        match campaign_mode {
            "Solo" => Ok(CampaignMode::Solo),
            "Coop" => Ok(CampaignMode::Coop),
            campaign_mode => Err(Error::HaloWaypointUnknownCampaignMode {
                campaign_mode: campaign_mode.to_string(),
            }),
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

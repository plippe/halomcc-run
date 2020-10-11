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
    fn from_str(it: &str) -> Result<Self, Self::Err> {
        match it {
            "Solo" => Ok(CampaignMode::Solo),
            "Coop" => Ok(CampaignMode::Coop),
            it => Err(Error::HaloWaypointUnknownCampaignMode {
                campaign_mode: it.to_string(),
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

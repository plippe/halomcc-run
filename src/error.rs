#[derive(Debug, Clone)]
pub enum Error {
    Hyper(String),
    HaloWaypoint(HaloWaypointError),
    List(Vec<Error>),
}

impl Error {
    pub fn from_hyper(err: hyper::Error) -> Self {
        Self::Hyper(format!("{:?}", err))
    }
}

#[derive(Debug, Clone)]
pub enum HaloWaypointError {
    Http(/* http::response::Parts, */ String),
    MissingGame,
    UnknownGame(String),
    MissingMissionId,
    InvalidMissionId(String),
    MissingDifficulty,
    UnknownDifficulty(String),
    MissingCampaignMode,
    UnknownCampaignMode(String),
    MissingTime,
    InvalidTime(String),
    MissingScore,
    InvalidScore(String),
}

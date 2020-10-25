use http::Response;

#[derive(Debug)]
pub enum HaloWaypointError {
    Http { response: Response<String> },
    MissingGame,
    UnknownGame { game: String },
    MissingMissionId,
    InvalidMissionId { mission_id: String },
    MissingDifficulty,
    UnknownDifficulty { difficulty: String },
    MissingCampaignMode,
    UnknownCampaignMode { campaign_mode: String },
    MissingTime,
    InvalidTime { time: String },
}

#[derive(Debug)]
pub enum Error {
    Hyper(hyper::Error),
    HaloWaypoint(HaloWaypointError),
    List { errors: Vec<Error> },
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(err)
    }
}

impl From<HaloWaypointError> for Error {
    fn from(err: HaloWaypointError) -> Self {
        Error::HaloWaypoint(err)
    }
}

#[derive(Debug, Clone)]
pub enum HaloWaypointError {
    Http {
        /*parts: http::response::Parts,*/ body: String,
    },
    MissingGame,
    UnknownGame {
        game: String,
    },
    MissingMissionId,
    InvalidMissionId {
        mission_id: String,
    },
    MissingDifficulty,
    UnknownDifficulty {
        difficulty: String,
    },
    MissingCampaignMode,
    UnknownCampaignMode {
        campaign_mode: String,
    },
    MissingTime,
    InvalidTime {
        time: String,
    },
    MissingScore,
    InvalidScore {
        score: String,
    },
}

#[derive(Debug, Clone)]
pub enum Error {
    Hyper(String),
    HaloWaypoint(HaloWaypointError),
    List { errors: Vec<Error> },
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper(format!("{:?}", err))
    }
}

impl From<HaloWaypointError> for Error {
    fn from(err: HaloWaypointError) -> Self {
        Error::HaloWaypoint(err)
    }
}

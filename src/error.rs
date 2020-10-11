#[derive(Debug)]
pub enum Error {
    HaloWaypointStatus { status: u16, body: String },
    HaloWaypointMissingGame,
    HaloWaypointUnknownGame { game: String },
    HaloWaypointMissingMissionId,
    HaloWaypointInvalidMissionId { mission_id: String },
    HaloWaypointMissingDifficulty,
    HaloWaypointUnknownDifficulty { difficulty: String },
    HaloWaypointMissingCampaignMode,
    HaloWaypointUnknownCampaignMode { campaign_mode: String },
    HaloWaypointMissingTime,
    HaloWaypointInvalidTime { time: String },
    Hyper { msg: String },
    ParseIntError { msg: String },
    List { errors: Vec<Error> },
}

impl From<hyper::Error> for Error {
    fn from(err: hyper::Error) -> Self {
        Error::Hyper {
            msg: format!("{:?}", err),
        }
    }
}

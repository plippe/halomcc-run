use http::method::Method;
use http::uri::{Builder, PathAndQuery, Scheme, Uri};
use http::{header, Request, Response, StatusCode};
use hyper::Body;
use scraper::{ElementRef, Html, Selector};
use std::convert::TryFrom;
use std::result::Result;
use time::Time;

use crate::campaign_modes::campaign_mode::CampaignMode as InternalCampaignMode;
use crate::chainable::Chainable;
use crate::difficulties::difficulty::Difficulty as InternalDifficulty;
use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game as InternalGame;
use crate::games::game::GameId;
use crate::halo_waypoint::models::campaign_mode::CampaignMode;
use crate::halo_waypoint::models::difficulty::Difficulty;
use crate::halo_waypoint::models::fastest_time::FastestTime;
use crate::halo_waypoint::models::game::Game;
use crate::halo_waypoint::models::highest_score::HighestScore;
use crate::halo_waypoint::models::mission_id::MissionId;
use crate::halo_waypoint::requests::auth::GetAuthResponse;

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct GetServiceRecordRequest {
    player: String,
    game: Game,
    campaign_mode: CampaignMode,
}

impl GetServiceRecordRequest {
    pub fn new(player: &str, game: &Game, campaign_mode: &CampaignMode) -> Self {
        Self {
            player: player.to_string(),
            game: *game,
            campaign_mode: *campaign_mode,
        }
    }

    pub fn from_internal(
        player: &str,
        game: &InternalGame,
        campaign_mode: &InternalCampaignMode,
    ) -> Self {
        Self::new(
            player,
            &Game::from_internal(game),
            &CampaignMode::from_internal(campaign_mode),
        )
    }
}

pub struct AuthenticatedGetServiceRecord {
    authentication: GetAuthResponse,
    request: GetServiceRecordRequest,
}

impl AuthenticatedGetServiceRecord {
    pub fn new(authentication: GetAuthResponse, request: GetServiceRecordRequest) -> Self {
        Self {
            authentication,
            request,
        }
    }

    pub fn to_uri(&self) -> Uri {
        let path_and_query = format!(
            "/{}/games/{}/{}/service-records/players/{}/missions?game={}&campaignMode={}",
            "en-us",                            // local
            "halo-the-master-chief-collection", // game
            "xbox-one",                         // platform
            self.request.player,
            self.request.game.to_string(),
            self.request.campaign_mode.to_string(),
        )
        .pipe(PathAndQuery::from_maybe_shared)
        .unwrap();

        Builder::new()
            .scheme(Scheme::HTTPS)
            .authority("www.halowaypoint.com")
            .path_and_query(path_and_query)
            .build()
            .unwrap()
    }

    pub fn to_request(&self) -> Request<Body> {
        Request::builder()
            .method(Method::GET)
            .uri(self.to_uri())
            .header(header::COOKIE, self.authentication.auth_header())
            .header("X-Requested-With", "XMLHttpRequest")
            .body(Body::empty())
            .unwrap()
    }
}

impl From<&AuthenticatedGetServiceRecord> for Request<Body> {
    fn from(req: &AuthenticatedGetServiceRecord) -> Self {
        req.to_request()
    }
}

#[cfg(test)]
mod get_service_record_request_test {
    use super::*;

    #[test]
    fn to_uri() {
        let req = AuthenticatedGetServiceRecord::new(
            GetAuthResponse::new("".to_string()),
            GetServiceRecordRequest::new("John117", &Game::Halo, &CampaignMode::Solo),
        );
        let uri = req.to_uri();

        assert_eq!(uri.path(), "/en-us/games/halo-the-master-chief-collection/xbox-one/service-records/players/John117/missions");
        assert_eq!(
            uri.query(),
            Some("game=HaloCombatEvolved&campaignMode=Solo")
        );
    }
}

#[derive(Debug, Clone)]
pub struct GetServiceRecordResponse {
    game: Game,
    campaign_mode: CampaignMode,
    missions: Vec<GetServiceRecordResponseMission>,
}

impl GetServiceRecordResponse {
    fn try_from_response(res: Response<String>) -> Result<Self, Error> {
        match res.status() {
            StatusCode::OK => Html::parse_fragment(res.body())
                .root_element()
                .pipe(Self::try_from_halo_waypoint_service_record),
            _ => HaloWaypointError::Http(res.into_body())
                .pipe(Error::HaloWaypoint)
                .pipe(Err),
        }
    }

    fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let game = Game::try_from_halo_waypoint_service_record(element);
        let campaign_mode = CampaignMode::try_from_halo_waypoint_service_record(element);

        let missions = Selector::parse("[data-mission-id]")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .map(GetServiceRecordResponseMission::try_from_halo_waypoint_service_record)
                    .collect::<Result<Vec<GetServiceRecordResponseMission>, Error>>()
            });

        match (game, campaign_mode, missions) {
            (Ok(game), Ok(campaign_mode), Ok(missions)) => Ok(Self {
                game,
                campaign_mode,
                missions,
            }),
            (game, campaign_mode, missions) => {
                vec![game.err(), campaign_mode.err(), missions.err()]
                    .into_iter()
                    .flatten()
                    .collect::<Vec<Error>>()
                    .pipe(Error::List)
                    .pipe(Err)
            }
        }
    }

    pub fn to_internal(
        &self,
    ) -> Vec<(
        GameId,
        i32,
        InternalCampaignMode,
        InternalDifficulty,
        Time,
        i32,
    )> {
        let game_id = self.game.to_internal();
        let campaign_mode = self.campaign_mode.to_internal();

        self.missions
            .iter()
            .filter_map(move |m| {
                let mission_id = m.id.to_internal();
                let difficulty = m.difficulty.to_internal();
                let time = m.fastest_time.to_internal();
                let score = m.highest_score.to_internal().unwrap_or(0);

                match (difficulty, time) {
                    (Some(difficulty), Some(time)) => {
                        Some((game_id, mission_id, campaign_mode, difficulty, time, score))
                    }
                    _ => None,
                }
            })
            .collect()
    }
}

impl TryFrom<Response<String>> for GetServiceRecordResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        Self::try_from_response(res)
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetServiceRecordResponseMission {
    id: MissionId,
    difficulty: Difficulty,
    fastest_time: FastestTime,
    highest_score: HighestScore,
}

impl GetServiceRecordResponseMission {
    fn try_from_halo_waypoint_service_record(element: ElementRef) -> Result<Self, Error> {
        let id = MissionId::try_from_halo_waypoint_service_record(element);
        let difficulty = Difficulty::try_from_halo_waypoint_service_record(element);
        let fastest_time = FastestTime::try_from_halo_waypoint_service_record(element);
        let highest_score = HighestScore::try_from_halo_waypoint_service_record(element);

        match (id, difficulty, fastest_time, highest_score) {
            (Ok(id), Ok(difficulty), Ok(fastest_time), Ok(highest_score)) => Ok(Self {
                id,
                difficulty,
                fastest_time,
                highest_score,
            }),
            (id, difficulty, fastest_time, highest_score) => vec![
                id.err(),
                difficulty.err(),
                fastest_time.err(),
                highest_score.err(),
            ]
            .into_iter()
            .flatten()
            .collect::<Vec<Error>>()
            .pipe(Error::List)
            .pipe(Err),
        }
    }
}

#[cfg(test)]
mod get_service_record_response_test {
    use super::*;
    use std::fs;
    use time::time;

    #[test]
    fn try_from_string() {
        let res = fs::read_dir("resources/halo_waypoint/service_records/")
            .unwrap()
            .map(|entry| {
                fs::read_to_string(entry.unwrap().path())
                    .unwrap()
                    .pipe(|s| Html::parse_fragment(&s))
                    .root_element()
                    .pipe(GetServiceRecordResponse::try_from_halo_waypoint_service_record)
            })
            .collect::<Result<Vec<GetServiceRecordResponse>, Error>>();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 12);
    }

    #[test]
    fn halo_solo() {
        let res = fs::read_to_string("resources/halo_waypoint/service_records/halo_solo.html")
            .unwrap()
            .pipe(|s| Html::parse_fragment(&s))
            .root_element()
            .pipe(GetServiceRecordResponse::try_from_halo_waypoint_service_record)
            .unwrap();

        assert_eq!(res.game, Game::Halo);
        assert_eq!(res.campaign_mode, CampaignMode::Solo);

        assert_eq!(
            res.missions.get(0),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(0),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:15:53)),
                highest_score: HighestScore::Some(23520),
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(1),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(01:27:34)),
                highest_score: HighestScore::None,
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(2),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:39:03)),
                highest_score: HighestScore::Some(6974),
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(3),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:20:47)),
                highest_score: HighestScore::Some(8204),
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(4),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:44:50)),
                highest_score: HighestScore::Some(10301),
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(5),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:18:56)),
                highest_score: HighestScore::Some(3601),
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(6),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:41:19)),
                highest_score: HighestScore::Some(11838),
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(7),
                difficulty: Difficulty::None,
                fastest_time: FastestTime::None,
                highest_score: HighestScore::None
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(8),
                difficulty: Difficulty::None,
                fastest_time: FastestTime::None,
                highest_score: HighestScore::None,
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(9),
                difficulty: Difficulty::Normal,
                fastest_time: FastestTime::Some(time!(00:39:46)),
                highest_score: HighestScore::Some(3319),
            })
        );

        assert_eq!(res.missions.get(10), None);
    }

    #[test]
    fn halo_coop() {
        let res = fs::read_to_string("resources/halo_waypoint/service_records/halo_coop.html")
            .unwrap()
            .pipe(|s| Html::parse_fragment(&s))
            .root_element()
            .pipe(GetServiceRecordResponse::try_from_halo_waypoint_service_record)
            .unwrap();

        assert_eq!(res.game, Game::Halo);
        assert_eq!(res.campaign_mode, CampaignMode::Coop);
        assert_eq!(
            res.missions.get(0),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(0),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:13:35)),
                highest_score: HighestScore::Some(19147),
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(1),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:35:13)),
                highest_score: HighestScore::Some(7953),
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(2),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:42:42)),
                highest_score: HighestScore::Some(23553),
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(3),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:27:46)),
                highest_score: HighestScore::Some(17378),
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(4),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:23:57)),
                highest_score: HighestScore::None,
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(5),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:29:31)),
                highest_score: HighestScore::Some(11021),
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(6),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:59:24)),
                highest_score: HighestScore::Some(44636),
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(7),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:57:49)),
                highest_score: HighestScore::Some(12172),
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(8),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:50:27)),
                highest_score: HighestScore::Some(16359),
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(9),
                difficulty: Difficulty::Legendary,
                fastest_time: FastestTime::Some(time!(00:40:42)),
                highest_score: HighestScore::Some(21823),
            })
        );

        assert_eq!(res.missions.get(10), None);
    }
}

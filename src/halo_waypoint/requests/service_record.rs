use http::method::Method;
use http::uri::{Builder, PathAndQuery, Scheme, Uri};
use http::{header, Request, Response, StatusCode};
use hyper::Body;
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::chainable::Chainable;
use crate::difficulties::difficulty::Difficulty;
use crate::error::{Error, HaloWaypointError};
use crate::games::game::Game;
use crate::halo_waypoint::models::fastest_time::FastestTime;
use crate::halo_waypoint::models::highest_score::HighestScore;
use crate::halo_waypoint::models::mission_id::MissionId;
use crate::halo_waypoint::requests::auth::GetAuthResponse;
use crate::service_records::service_record::{ServiceRecord, ServiceRecordRun};

#[derive(PartialEq, Eq, Hash, Clone)]
pub struct GetServiceRecordRequest {
    player: String,
    game: Game,
    campaign_mode: CampaignMode,
}

impl GetServiceRecordRequest {
    pub fn new(player: String, game: Game, campaign_mode: CampaignMode) -> Self {
        Self {
            player,
            game,
            campaign_mode,
        }
    }
}

pub struct GetServiceRecordRequestAuthenticated {
    auth_header: String,
    player: String,
    game: Game,
    campaign_mode: CampaignMode,
}

impl GetServiceRecordRequestAuthenticated {
    pub fn new(
        auth_header: String,
        player: String,
        game: Game,
        campaign_mode: CampaignMode,
    ) -> Self {
        Self {
            auth_header,
            player,
            game,
            campaign_mode,
        }
    }
}

impl From<(&GetAuthResponse, &GetServiceRecordRequest)> for GetServiceRecordRequestAuthenticated {
    fn from(req: (&GetAuthResponse, &GetServiceRecordRequest)) -> Self {
        Self::new(
            req.0.auth_header(),
            req.1.player.clone(),
            req.1.game,
            req.1.campaign_mode,
        )
    }
}

impl From<&GetServiceRecordRequestAuthenticated> for Uri {
    fn from(req: &GetServiceRecordRequestAuthenticated) -> Self {
        let path_and_query = format!(
            "/{}/games/{}/{}/service-records/players/{}/missions?game={}&campaignMode={}",
            "en-us",                            // local
            "halo-the-master-chief-collection", // game
            "xbox-one",                         // platform
            req.player,
            req.game.to_string(),
            req.campaign_mode.to_string(),
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
}

impl From<&GetServiceRecordRequestAuthenticated> for Request<Body> {
    fn from(req: &GetServiceRecordRequestAuthenticated) -> Self {
        Request::builder()
            .method(Method::GET)
            .uri(Uri::from(req))
            .header(header::COOKIE, req.auth_header.clone())
            .header("X-Requested-With", "XMLHttpRequest")
            .body(Body::empty())
            .unwrap()
    }
}

#[cfg(test)]
mod get_service_record_request_test {
    use super::*;

    #[test]
    fn into_uri() {
        let req = GetServiceRecordRequestAuthenticated::new(
            "".to_string(),
            "John117".to_string(),
            Game::Halo,
            CampaignMode::Solo,
        );
        let uri = Uri::from(&req);

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

impl TryFrom<Response<String>> for GetServiceRecordResponse {
    type Error = Error;
    fn try_from(res: Response<String>) -> Result<Self, Self::Error> {
        match res.status() {
            StatusCode::OK => Html::parse_fragment(res.body()).pipe(Self::try_from),
            _ => Err(HaloWaypointError::Http {
                body: res.into_body(),
            }
            .into()),
        }
    }
}

impl TryFrom<Html> for GetServiceRecordResponse {
    type Error = Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        html.root_element().pipe(Self::try_from)
    }
}

impl<'a> TryFrom<ElementRef<'a>> for GetServiceRecordResponse {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let game = Selector::parse("[data-game-id]")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .next()
                    .and_then(|element| element.value().attr("data-game-id"))
                    .ok_or_else(|| HaloWaypointError::MissingGame.into())
            })
            .and_then(Game::from_str);

        let campaign_mode = Selector::parse("[data-mode-id]")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .next()
                    .and_then(|element| element.value().attr("data-mode-id"))
                    .ok_or_else(|| HaloWaypointError::MissingCampaignMode.into())
            })
            .and_then(CampaignMode::from_str);

        let missions = Selector::parse("[data-mission-id]")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .map(GetServiceRecordResponseMission::try_from)
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
                    .pipe(|errors| Error::List { errors })
                    .pipe(Err)
            }
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct GetServiceRecordResponseMission {
    id: MissionId,
    difficulty: Option<Difficulty>,
    fastest_time: FastestTime,
    highest_score: HighestScore,
}

impl<'a> TryFrom<ElementRef<'a>> for GetServiceRecordResponseMission {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let id = MissionId::try_from(element);

        let difficulty = Selector::parse(".skull .spritesheet")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .next()
                    .and_then(|element| element.value().attr("title"))
            })
            .ok_or_else(|| HaloWaypointError::MissingDifficulty.into())
            .and_then(|attribute| match attribute {
                "None" => Ok(None),
                attribute => attribute.parse().map(Some),
            });

        let fastest_time = FastestTime::try_from(element);
        let highest_score = HighestScore::try_from(element);

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
            .pipe(|errors| Error::List { errors })
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
                    .pipe(|s| Html::parse_fragment(&s).pipe(GetServiceRecordResponse::try_from))
            })
            .collect::<Result<Vec<GetServiceRecordResponse>, Error>>();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 12);
    }

    #[test]
    fn halo_solo() {
        let res = fs::read_to_string("resources/halo_waypoint/service_records/halo_solo.html")
            .unwrap()
            .pipe(|s| Html::parse_fragment(&s).pipe(GetServiceRecordResponse::try_from))
            .unwrap();

        assert_eq!(res.game, Game::Halo);
        assert_eq!(res.campaign_mode, CampaignMode::Solo);

        assert_eq!(
            res.missions.get(0),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(0),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:15:53)),
                highest_score: HighestScore::new(23520),
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(1),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(01:27:34)),
                highest_score: HighestScore::empty(),
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(2),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:39:03)),
                highest_score: HighestScore::new(6974),
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(3),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:20:47)),
                highest_score: HighestScore::new(8204),
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(4),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:44:50)),
                highest_score: HighestScore::new(10301),
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(5),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:18:56)),
                highest_score: HighestScore::new(3601),
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(6),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:41:19)),
                highest_score: HighestScore::new(11838),
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(7),
                difficulty: None,
                fastest_time: FastestTime::empty(),
                highest_score: HighestScore::empty()
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(8),
                difficulty: None,
                fastest_time: FastestTime::empty(),
                highest_score: HighestScore::empty(),
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(9),
                difficulty: Some(Difficulty::Normal),
                fastest_time: FastestTime::new(time!(00:39:46)),
                highest_score: HighestScore::new(3319),
            })
        );

        assert_eq!(res.missions.get(10), None);
    }

    #[test]
    fn halo_coop() {
        let res = fs::read_to_string("resources/halo_waypoint/service_records/halo_coop.html")
            .unwrap()
            .pipe(|s| Html::parse_fragment(&s).pipe(GetServiceRecordResponse::try_from))
            .unwrap();

        assert_eq!(res.game, Game::Halo);
        assert_eq!(res.campaign_mode, CampaignMode::Coop);
        assert_eq!(
            res.missions.get(0),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(0),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:13:35)),
                highest_score: HighestScore::new(19147),
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(1),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:35:13)),
                highest_score: HighestScore::new(7953),
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(2),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:42:42)),
                highest_score: HighestScore::new(23553),
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(3),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:27:46)),
                highest_score: HighestScore::new(17378),
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(4),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:23:57)),
                highest_score: HighestScore::empty(),
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(5),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:29:31)),
                highest_score: HighestScore::new(11021),
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(6),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:59:24)),
                highest_score: HighestScore::new(44636),
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(7),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:57:49)),
                highest_score: HighestScore::new(12172),
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(8),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:50:27)),
                highest_score: HighestScore::new(16359),
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: MissionId::new(9),
                difficulty: Some(Difficulty::Legendary),
                fastest_time: FastestTime::new(time!(00:40:42)),
                highest_score: HighestScore::new(21823),
            })
        );

        assert_eq!(res.missions.get(10), None);
    }
}

pub struct PlayerWithGetServiceRecordResponse {
    player: String,
    responses: Vec<GetServiceRecordResponse>,
}

impl From<(String, Vec<GetServiceRecordResponse>)> for PlayerWithGetServiceRecordResponse {
    fn from(
        tuple_player_responses: (String, Vec<GetServiceRecordResponse>),
    ) -> PlayerWithGetServiceRecordResponse {
        let (player, responses) = tuple_player_responses;
        PlayerWithGetServiceRecordResponse { player, responses }
    }
}

impl Into<Vec<ServiceRecord>> for PlayerWithGetServiceRecordResponse {
    fn into(self) -> Vec<ServiceRecord> {
        let player = self.player.clone();
        self.responses
            .into_iter()
            .flat_map(|r| {
                let game_id = r.game.id();
                let missions_id_delta = r.game.missions_id_delta();
                let campaign_mode = r.campaign_mode;

                r.missions.into_iter().filter_map(move |m| {
                    match (m.difficulty, m.fastest_time.value()) {
                        (Some(difficulty), Some(time)) => Some((
                            (game_id, missions_id_delta + m.id.value()),
                            (
                                campaign_mode,
                                difficulty,
                                time,
                                m.highest_score.value().unwrap_or(0),
                            ),
                        )),
                        _ => None,
                    }
                })
            })
            .into_group_map()
            .into_iter()
            .map(|((game_id, mission_id), runs)| {
                let runs = runs
                    .into_iter()
                    .map(|(c, d, t, s)| ServiceRecordRun::new(c, d, t, s))
                    .collect();

                ServiceRecord::new(player.clone(), game_id, mission_id, runs)
            })
            .sorted()
            .collect()
    }
}

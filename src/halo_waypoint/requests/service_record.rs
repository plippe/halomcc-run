use http::method::Method;
use http::uri::{Builder, PathAndQuery, Scheme, Uri};
use http::{header, Request, Response, StatusCode};
use hyper::Body;
use itertools::Itertools;
use scraper::{ElementRef, Html, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;
use time::Time;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::chainable::Chainable;
use crate::difficulties::difficulty::Difficulty;
use crate::error::{Error, HaloWaypointError};
use crate::games::game::{Game, GameProperties};
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

impl GetServiceRecordResponse {
    pub fn game(&self) -> Game {
        self.game
    }

    pub fn campaign_mode(&self) -> CampaignMode {
        self.campaign_mode
    }

    pub fn missions(&self) -> Vec<GetServiceRecordResponseMission> {
        self.missions.clone()
    }
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
    id: u8,
    difficulty: Option<Difficulty>,
    time: Option<Time>,
}

impl GetServiceRecordResponseMission {
    pub fn id(&self) -> u8 {
        self.id
    }

    pub fn difficulty(&self) -> Option<Difficulty> {
        self.difficulty
    }

    pub fn time(&self) -> Option<Time> {
        self.time
    }
}

impl<'a> TryFrom<ElementRef<'a>> for GetServiceRecordResponseMission {
    type Error = Error;
    fn try_from(element: ElementRef) -> Result<Self, Self::Error> {
        let id = element
            .value()
            .attr("data-mission-id")
            .ok_or(HaloWaypointError::MissingMissionId)
            .and_then(|attribute| {
                attribute
                    .parse()
                    .map_err(|_| HaloWaypointError::InvalidMissionId {
                        mission_id: attribute.to_string(),
                    })
            })
            .map_err(|err| err.into());

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

        let time =
            Selector::parse(".best-time")
                .unwrap()
                .pipe(|selector| {
                    element
                        .select(&selector)
                        .next()
                        .ok_or(HaloWaypointError::MissingTime)
                })
                .and_then(|element| match element.inner_html().as_str() {
                    "--" => Ok(None),
                    html => Time::parse(html, "%T").map(Some).map_err(|_| {
                        HaloWaypointError::InvalidTime {
                            time: html.to_string(),
                        }
                    }),
                })
                .map_err(|err| err.into());

        match (id, difficulty, time) {
            (Ok(id), Ok(difficulty), Ok(time)) => Ok(Self {
                id,
                difficulty,
                time,
            }),
            (id, difficulty, time) => vec![id.err(), difficulty.err(), time.err()]
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
                id: 0,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:15:53))
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: 1,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(01:27:34))
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: 2,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:39:03))
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: 3,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:20:47))
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: 4,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:44:50))
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: 5,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:18:56))
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: 6,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:41:19))
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: 7,
                difficulty: None,
                time: None,
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: 8,
                difficulty: None,
                time: None,
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: 9,
                difficulty: Some(Difficulty::Normal),
                time: Some(time!(00:39:46))
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
                id: 0,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:13:35))
            })
        );

        assert_eq!(
            res.missions.get(1),
            Some(&GetServiceRecordResponseMission {
                id: 1,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:35:13))
            })
        );

        assert_eq!(
            res.missions.get(2),
            Some(&GetServiceRecordResponseMission {
                id: 2,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:42:42))
            })
        );

        assert_eq!(
            res.missions.get(3),
            Some(&GetServiceRecordResponseMission {
                id: 3,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:27:46))
            })
        );

        assert_eq!(
            res.missions.get(4),
            Some(&GetServiceRecordResponseMission {
                id: 4,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:23:57))
            })
        );

        assert_eq!(
            res.missions.get(5),
            Some(&GetServiceRecordResponseMission {
                id: 5,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:29:31))
            })
        );

        assert_eq!(
            res.missions.get(6),
            Some(&GetServiceRecordResponseMission {
                id: 6,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:59:24))
            })
        );

        assert_eq!(
            res.missions.get(7),
            Some(&GetServiceRecordResponseMission {
                id: 7,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:57:49))
            })
        );

        assert_eq!(
            res.missions.get(8),
            Some(&GetServiceRecordResponseMission {
                id: 8,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:50:27))
            })
        );

        assert_eq!(
            res.missions.get(9),
            Some(&GetServiceRecordResponseMission {
                id: 9,
                difficulty: Some(Difficulty::Legendary),
                time: Some(time!(00:40:42))
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
                let game_id = GameProperties::from(r.game()).id();
                let missions_id_delta = r.game().missions_id_delta();
                let campaign_mode = r.campaign_mode();

                r.missions()
                    .into_iter()
                    .filter_map(move |m| match (m.difficulty(), m.time()) {
                        (Some(difficulty), Some(time)) => Some((
                            (game_id, missions_id_delta + m.id() as i32),
                            (campaign_mode, difficulty, time),
                        )),
                        _ => None,
                    })
            })
            .into_group_map()
            .into_iter()
            .map(|((game_id, mission_id), runs)| {
                let runs = runs
                    .into_iter()
                    .map(|(c, d, t)| ServiceRecordRun::new(c, d, t))
                    .collect();

                ServiceRecord::new(player.clone(), game_id, mission_id, runs)
            })
            .collect()
    }
}

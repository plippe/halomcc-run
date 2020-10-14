use http::method::Method;
use http::uri::{Builder, PathAndQuery, Scheme, Uri};
use http::Request;
use hyper::Body;
use scraper::{ElementRef, Html, Selector};
use std::convert::TryFrom;
use std::result::Result;
use std::str::FromStr;
use time::Time;

use crate::chainable::Chainable;
use crate::error::{Error, HaloWaypointError};
use crate::halo_waypoint::models::campaign_mode::CampaignMode;
use crate::halo_waypoint::models::difficulty::Difficulty;
use crate::halo_waypoint::models::game::Game;

pub struct GetServiceRecordRequest {
    player: String,
    game: Game,
    campaign_mode: CampaignMode,
}

impl GetServiceRecordRequest {
    pub fn new(player: String, game: Game, campaign_mode: CampaignMode) -> GetServiceRecordRequest {
        GetServiceRecordRequest {
            player,
            game,
            campaign_mode,
        }
    }
}

impl From<&GetServiceRecordRequest> for Uri {
    fn from(req: &GetServiceRecordRequest) -> Uri {
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

impl From<&GetServiceRecordRequest> for Method {
    fn from(_: &GetServiceRecordRequest) -> Method {
        Method::GET
    }
}

impl From<&GetServiceRecordRequest> for Request<Body> {
    fn from(req: &GetServiceRecordRequest) -> Request<Body> {
        Request::builder()
            .method(Method::from(req))
            .uri(Uri::from(req))
            .header("user-agent", "halomcc.run/0.1")
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
        let req = GetServiceRecordRequest::new(
            "John117".to_string(),
            Game::HaloCombatEvolved,
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

#[derive(Debug)]
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

impl FromStr for GetServiceRecordResponse {
    type Err = Error;
    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Html::parse_fragment(s).pipe(GetServiceRecordResponse::try_from)
    }
}

impl TryFrom<Html> for GetServiceRecordResponse {
    type Error = Error;
    fn try_from(html: Html) -> Result<Self, Self::Error> {
        html.root_element().pipe(GetServiceRecordResponse::try_from)
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
                    .ok_or(HaloWaypointError::MissingGame.into())
            })
            .and_then(Game::from_str);

        let campaign_mode = Selector::parse("[data-mode-id]")
            .unwrap()
            .pipe(|selector| {
                element
                    .select(&selector)
                    .next()
                    .and_then(|element| element.value().attr("data-mode-id"))
                    .ok_or(HaloWaypointError::MissingCampaignMode.into())
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
            (Ok(game), Ok(campaign_mode), Ok(missions)) => Ok(GetServiceRecordResponse {
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
            .ok_or(HaloWaypointError::MissingDifficulty.into())
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
            (Ok(id), Ok(difficulty), Ok(time)) => Ok(GetServiceRecordResponseMission {
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
                    .pipe(|s| GetServiceRecordResponse::from_str(&s))
            })
            .collect::<Result<Vec<GetServiceRecordResponse>, Error>>();

        assert!(res.is_ok());
        assert_eq!(res.unwrap().len(), 12);
    }

    #[test]
    fn halo_solo() {
        let res = fs::read_to_string("resources/halo_waypoint/service_records/halo_solo.html")
            .unwrap()
            .pipe(|s| GetServiceRecordResponse::from_str(&s))
            .unwrap();

        assert_eq!(res.game, Game::HaloCombatEvolved);
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
            .pipe(|s| GetServiceRecordResponse::from_str(&s))
            .unwrap();

        assert_eq!(res.game, Game::HaloCombatEvolved);
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

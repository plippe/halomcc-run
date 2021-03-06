use futures::future::join_all;
use time::Time;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::chainable::Chainable;
use crate::difficulties::difficulty::Difficulty;
use crate::error::Error;
use crate::games::dao::{GamesDao, InMemoryGamesDao};
use crate::games::game::{Game, GameId};
use crate::halo_waypoint::client::{Client, InMemoryCacheClient};
use crate::halo_waypoint::requests::auth::GetAuthRequest;
use crate::halo_waypoint::requests::service_record::{
    GetServiceRecordRequest, GetServiceRecordResponse,
};
use crate::missions::mission::{Mission, MissionId};
use crate::service_records::service_record::ServiceRecord;

pub struct ServiceRecordsDao {
    games_dao: Box<dyn GamesDao + Send + Sync>,
    halo_waypoint: Box<dyn Client + Send + Sync>,
}

impl ServiceRecordsDao {
    pub async fn find_by_player_and_game(
        &self,
        player: String,
        game: Game,
    ) -> Option<Vec<ServiceRecord>> {
        let req = GetAuthRequest::default();
        let auth = self
            .halo_waypoint
            .get_auth(&req)
            .await
            .map_err(|err| eprintln!("{:?}", err))
            .ok()?;

        vec![CampaignMode::Solo, CampaignMode::Coop]
            .into_iter()
            .map(|campaign_mode| {
                let auth = auth.clone();
                let req =
                    GetServiceRecordRequest::from_internal(player.as_str(), &game, &campaign_mode);

                async move { self.halo_waypoint.get_service_record(&auth, &req).await }
            })
            .pipe(join_all)
            .await
            .into_iter()
            .collect::<Result<Vec<GetServiceRecordResponse>, Error>>()
            .map(|res| {
                res.iter()
                    .flat_map(GetServiceRecordResponse::to_internal)
                    .collect::<Vec<(GameId, MissionId, CampaignMode, Difficulty, Time, i32)>>()
                    .pipe(|runs| ServiceRecord::from_player_and_runs(&player, &runs))
            })
            .map_err(|err| eprintln!("{:?}", err))
            .ok()
    }

    async fn find_by_player_and_game_id(
        &self,
        player: String,
        game_id: GameId,
    ) -> Option<Vec<ServiceRecord>> {
        let game = self.games_dao.find_by_id(game_id)?;
        self.find_by_player_and_game(player, game).await
    }

    pub async fn find_by_player_and_mission(
        &self,
        player: String,
        mission: &Mission,
    ) -> Option<ServiceRecord> {
        self.find_by_player_and_game_id(player.clone(), mission.game_id())
            .await
            .and_then(|service_records| {
                service_records.into_iter().find(|service_record| {
                    service_record.player() == player
                        && service_record.game_id() == mission.game_id()
                        && service_record.mission_id() == mission.id()
                })
            })
    }

    pub fn default() -> Self {
        Self {
            games_dao: Box::new(InMemoryGamesDao::default()),
            halo_waypoint: Box::new(InMemoryCacheClient::default()),
        }
    }
}

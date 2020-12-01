use futures::future::join_all;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::chainable::Chainable;
use crate::error::Error;
use crate::games::dao::GamesDao;
use crate::games::game::Game;
use crate::halo_waypoint::client::{Client, HyperClient, InMemoryCacheClient};
use crate::halo_waypoint::requests::auth::GetAuthRequest;
use crate::halo_waypoint::requests::service_record::{
    GetServiceRecordRequest, GetServiceRecordResponse, PlayerWithGetServiceRecordResponse,
};
use crate::missions::mission::Mission;
use crate::service_records::service_record::ServiceRecord;

pub struct ServiceRecordsDao {
    games_dao: GamesDao,
    halo_waypoint: InMemoryCacheClient<HyperClient>,
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
                let req = GetServiceRecordRequest::new(player.clone(), game, campaign_mode);

                async move { self.halo_waypoint.get_service_record(&auth, &req).await }
            })
            .pipe(join_all)
            .await
            .into_iter()
            .collect::<Result<Vec<GetServiceRecordResponse>, Error>>()
            .map(|res| PlayerWithGetServiceRecordResponse::from((player.clone(), res)).into())
            .map_err(|err| eprintln!("{:?}", err))
            .ok()
    }

    async fn find_by_player_and_game_id(
        &self,
        player: String,
        game_id: i32,
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
}

impl Default for ServiceRecordsDao {
    fn default() -> Self {
        ServiceRecordsDao {
            games_dao: GamesDao::default(),
            halo_waypoint: InMemoryCacheClient::default(),
        }
    }
}

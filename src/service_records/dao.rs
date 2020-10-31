use crate::games::dao::GamesDao;
use crate::games::game::{Game, GameProperties};
use crate::halo_waypoint::client::{Client, HyperClient, InMemoryCacheClient};
use crate::halo_waypoint::models::campaign_mode::CampaignMode;
use crate::halo_waypoint::requests::auth::GetAuthRequest;
use crate::halo_waypoint::requests::service_record::GetServiceRecordRequest;
use crate::missions::mission::{Mission, MissionProperties};
use crate::service_records::service_record::ServiceRecord;

pub struct ServiceRecordsDao {
    games_dao: GamesDao,
    halo_waypoint: InMemoryCacheClient<HyperClient>,
}

impl ServiceRecordsDao {
    pub async fn find_by_player_and_game(
        &self,
        player: String,
        game: &Game,
    ) -> Option<Vec<ServiceRecord>> {
        let req = GetAuthRequest::default();
        let auth = self
            .halo_waypoint
            .get_auth(&req)
            .await
            .map_err(|err| eprintln!("{:?}", err))
            .ok()?;

        let req = GetServiceRecordRequest::new(player.clone(), game.into(), CampaignMode::Solo);
        self.halo_waypoint
            .get_service_record(&auth, &req)
            .await
            .map(|res| {
                let game: Game = res.game().into();
                res.missions()
                    .into_iter()
                    .map(|mission| {
                        ServiceRecord::new(
                            player.clone(),
                            GameProperties::from(&game).id(),
                            mission.id() as i32,
                            mission.time(),
                        )
                    })
                    .collect()
            })
            .map_err(|err| eprintln!("{:?}", err))
            .ok()
    }

    async fn find_by_player_and_game_id(
        &self,
        player: String,
        game_id: i32,
    ) -> Option<Vec<ServiceRecord>> {
        let game = self.games_dao.find_by_id(game_id)?;
        self.find_by_player_and_game(player, &game).await
    }

    pub async fn find_by_player_and_mission(
        &self,
        player: String,
        mission: &Mission,
    ) -> Option<ServiceRecord> {
        let properties = MissionProperties::from(mission);
        self.find_by_player_and_game_id(player.clone(), properties.game_id())
            .await
            .and_then(|service_records| {
                service_records.into_iter().find(|service_record| {
                    service_record.player() == player
                        && service_record.game_id() == properties.game_id()
                        && service_record.mission_id() == properties.id()
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

use juniper::graphql_object;

use crate::campaign_modes::campaign_mode::CampaignMode;
use crate::difficulties::difficulty::Difficulty;
use crate::games::game::Game;
use crate::graphql::context::Context;
use crate::graphql::models::time::Time;
use crate::missions::mission::Mission;
use crate::service_records::service_record::{ServiceRecord, ServiceRecordRun};

#[graphql_object(Context = Context)]
impl ServiceRecord {
    fn player(&self) -> String {
        self.player()
    }

    fn game_id(&self) -> i32 {
        *self.game_id().value()
    }

    fn game(&self, context: &Context) -> Option<Game> {
        context.games_dao().find_by_id(self.game_id())
    }

    fn mission_id(&self) -> i32 {
        self.mission_id()
    }

    fn mission(&self, context: &Context) -> Option<Mission> {
        context
            .missions_dao()
            .find_by_game_id_and_id(self.game_id(), self.mission_id())
    }

    fn runs(&self) -> Vec<ServiceRecordRun> {
        self.runs()
    }
}

#[graphql_object(Context = Context)]
impl ServiceRecordRun {
    fn campaign_mode(&self) -> CampaignMode {
        self.campaign_mode()
    }

    fn difficulty(&self) -> Difficulty {
        self.difficulty()
    }

    fn time_in_seconds(&self) -> Time {
        Time::from_time(&self.time())
    }

    fn score(&self) -> i32 {
        self.score()
    }
}

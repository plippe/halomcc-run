use juniper::graphql_object;

use crate::graphql::context::Context;
use crate::graphql::models::time::Time;
use crate::missions::mission::Mission;
use crate::service_records::service_record::ServiceRecord;

#[graphql_object(Context = Context)]
impl Mission {
    fn id(&self) -> i32 {
        self.id()
    }

    fn name(&self) -> String {
        self.name()
    }

    // TODO: GraphQL shouldn't require explicit conversions
    fn par_time_in_seconds(&self) -> Option<i32> {
        self.par_time()
            .map(|par_time| Time::from_time(&par_time).seconds())
    }

    fn par_score(&self) -> Option<i32> {
        self.par_score()
    }

    async fn service_record_by_player(
        &self,
        player: String,
        context: &Context,
    ) -> Option<ServiceRecord> {
        context
            .service_records_doa()
            .find_by_player_and_mission(player, self)
            .await
    }
}

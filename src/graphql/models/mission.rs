use juniper::graphql_object;

use crate::graphql::context::Context;
use crate::graphql::models::time::Time;
use crate::missions::mission::{Mission, MissionProperties};
use crate::service_records::service_record::ServiceRecord;

#[graphql_object(Context = Context)]
impl Mission {
    fn id(&self) -> i32 {
        MissionProperties::from(self).id()
    }

    fn name(&self) -> String {
        MissionProperties::from(self).name()
    }

    // TODO: GraphQL shouldn't require explicit conversions
    fn par_time_in_seconds(&self) -> Option<i32> {
        MissionProperties::from(self)
            .par_time()
            .map(|par_time| Time::from(par_time).seconds())
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

use juniper::graphql_object;

use crate::graphql::context::Context;
use crate::graphql::time::Time;
use crate::service_records::service_record::ServiceRecord;

#[graphql_object(Context = Context)]
impl ServiceRecord {
    fn player(&self) -> String {
        self.player()
    }

    fn game_id(&self) -> i32 {
        self.game_id()
    }

    fn mission_id(&self) -> i32 {
        self.mission_id()
    }

    // TODO: GraphQL shouldn't require explicit conversions
    fn time_in_seconds(&self) -> Option<i32> {
        self.time().map(|time| Time::from(time).seconds())
    }

    fn difficulty(&self) -> Option<String> {
        self.difficulty().map(|difficulty| difficulty.to_string())
    }
}

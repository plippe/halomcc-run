use juniper::graphql_object;

use crate::games::game::{Game, GameProperties};
use crate::graphql::context::Context;
use crate::missions::mission::Mission;
use crate::service_records::service_record::ServiceRecord;

#[graphql_object(Context = Context)]
impl Game {
    fn id(&self) -> i32 {
        GameProperties::from(*self).id()
    }

    fn name(&self) -> String {
        GameProperties::from(*self).name()
    }

    fn missions(&self, context: &Context) -> Vec<Mission> {
        context
            .missions_dao()
            .all_by_game_id(GameProperties::from(*self).id())
    }

    async fn service_record_by_player(
        &self,
        player: String,
        context: &Context,
    ) -> Option<Vec<ServiceRecord>> {
        context
            .service_records_doa()
            .find_by_player_and_game(player, *self)
            .await
    }
}

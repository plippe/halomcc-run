use juniper::graphql_object;

use crate::games::Game;
use crate::graphql::Context;
use crate::levels::Level;
use crate::service_records::ServiceRecord;

#[graphql_object(Context = Context)]
impl ServiceRecord {
    fn player(&self) -> String {
        self.player.clone()
    }

    fn game_id(&self) -> i32 {
        self.game_id
    }

    fn level_id(&self) -> i32 {
        self.level_id
    }

    fn best_time(&self) -> i32 {
        self.best_time
    }

    fn game(&self, context: &Context) -> Option<Game> {
        context.games_dao.find_by_id(self.game_id)
    }

    fn level(&self, context: &Context) -> Option<Level> {
        context
            .levels_dao
            .find_by_game_id_and_id(self.game_id, self.level_id)
    }
}

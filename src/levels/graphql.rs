use juniper::graphql_object;

use crate::games::Game;
use crate::graphql::Context;
use crate::levels::Level;

#[graphql_object(Context = Context)]
impl Level {
    fn game_id(&self) -> i32 {
        self.game_id
    }

    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn par_time(&self) -> i32 {
        self.par_time
    }

    fn game(&self, context: &Context) -> Option<Game> {
        context.games_dao.find_by_id(self.game_id)
    }
}

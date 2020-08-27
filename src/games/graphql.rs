use juniper::graphql_object;

use crate::games::Game;
use crate::graphql::Context;
use crate::levels::Level;

#[graphql_object(Context = Context)]
impl Game {
    fn id(&self) -> i32 {
        self.id
    }

    fn name(&self) -> &str {
        self.name.as_str()
    }

    fn levels(&self, context: &Context) -> Vec<Level> {
        context.levels_dao.all_by_game_id(self.id)
    }
}

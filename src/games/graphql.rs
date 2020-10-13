use juniper::graphql_object;

use crate::games::game::{Game, GameProperties};
use crate::graphql::context::Context;
use crate::missions::mission::Mission;

#[graphql_object(Context = Context)]
impl Game {
    fn id(&self) -> i32 {
        GameProperties::from(self).id()
    }

    fn name(&self) -> String {
        GameProperties::from(self).name()
    }

    fn missions(&self, context: &Context) -> Vec<Mission> {
        context
            .missions_dao
            .all_by_game_id(GameProperties::from(self).id())
    }
}

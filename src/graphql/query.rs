use juniper::{graphql_object, FieldResult};

use crate::games::game::Game;
use crate::graphql::context::Context;

pub struct Query;

#[graphql_object(Context = Context)]
impl Query {
    fn games(context: &Context) -> FieldResult<Vec<Game>> {
        Ok(context.games_dao.all())
    }

    fn game(context: &Context, id: i32) -> FieldResult<Option<Game>> {
        Ok(context.games_dao.find_by_id(id))
    }
}

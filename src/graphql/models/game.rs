use juniper::{graphql_object, ParseScalarResult, Value};

use crate::games::game::{Game, GameId};
use crate::graphql::context::Context;
use crate::missions::mission::Mission;
use crate::service_records::service_record::ServiceRecord;

#[juniper::graphql_scalar(description = "Game id")]
impl<S> GraphQLScalar for GameId
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(*self.value())
    }

    fn from_input_value(_v: &InputValue) -> Option<Self> {
        unimplemented!();
    }

    fn from_str<'a>(_value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        unimplemented!();
    }
}

#[graphql_object(Context = Context)]
impl Game {
    fn id(&self) -> GameId {
        self.id()
    }

    fn name(&self) -> String {
        self.name()
    }

    fn missions(&self, context: &Context) -> Vec<Mission> {
        context.missions_dao().all_by_game_id(self.id())
    }

    async fn service_record_by_player(
        &self,
        player: String,
        context: &Context,
    ) -> Option<Vec<ServiceRecord>> {
        context
            .service_records_doa()
            .find_by_player_and_game(player, self.clone())
            .await
    }
}

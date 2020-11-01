use juniper::{ParseScalarResult, Value};

use crate::difficulties::difficulty::Difficulty;

#[juniper::graphql_scalar(description = "Difficulty")]
impl<S> GraphQLScalar for Difficulty
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        let value = match self {
            Difficulty::Easy => "easy",
            Difficulty::Normal => "normal",
            Difficulty::Heroic => "heroic",
            Difficulty::Legendary => "legendary",
        };

        Value::scalar(value.to_string())
    }

    fn from_input_value(_v: &InputValue) -> Option<Difficulty> {
        unimplemented!();
    }

    fn from_str<'a>(_value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        unimplemented!();
    }
}

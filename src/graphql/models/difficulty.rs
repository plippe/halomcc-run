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
            Self::Easy => "easy",
            Self::Normal => "normal",
            Self::Heroic => "heroic",
            Self::Legendary => "legendary",
        };

        Value::scalar(value.to_string())
    }

    fn from_input_value(_v: &InputValue) -> Option<Self> {
        unimplemented!();
    }

    fn from_str<'a>(_value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        unimplemented!();
    }
}

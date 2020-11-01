use juniper::{ParseScalarResult, Value};

use crate::campaign_modes::campaign_mode::CampaignMode;

#[juniper::graphql_scalar(description = "CampaignMode")]
impl<S> GraphQLScalar for CampaignMode
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        let value = match self {
            CampaignMode::Solo => "solo",
            CampaignMode::Coop => "coop",
        };

        Value::scalar(value.to_string())
    }

    fn from_input_value(_v: &InputValue) -> Option<CampaignMode> {
        unimplemented!();
    }

    fn from_str<'a>(_value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        unimplemented!();
    }
}

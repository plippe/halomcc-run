use juniper::{ParseScalarResult, Value};

pub struct Time(time::Time);

impl Time {
    pub fn seconds(&self) -> i32 {
        let hours = self.0.hour() as i32;
        let minutes = hours * 60 + self.0.minute() as i32;
        minutes * 60 + self.0.second() as i32
    }
}

impl From<time::Time> for Time {
    fn from(time: time::Time) -> Time {
        Time(time)
    }
}

#[cfg(test)]
mod time_test {
    use super::*;

    #[test]
    fn min() {
        assert_eq!(Time(time::time!(00:00:00.000_000_000)).seconds(), 0)
    }

    #[test]
    fn max() {
        assert_eq!(Time(time::time!(23:59:59.999_999_999)).seconds(), 86399)
    }
}

#[juniper::graphql_scalar(description = "Time")]
impl<S> GraphQLScalar for Time
where
    S: ScalarValue,
{
    // Define how to convert your custom scalar into a primitive type.
    fn resolve(&self) -> Value {
        Value::scalar(self.seconds())
    }

    fn from_input_value(_v: &InputValue) -> Option<Time> {
        unimplemented!();
    }

    fn from_str<'a>(_value: ScalarToken<'a>) -> ParseScalarResult<'a, S> {
        unimplemented!();
    }
}

use std::ops::Sub;

use chrono::{Duration, Offset, TimeZone};
use serde_json::Value;

use super::{
    logic::{self, parse_float},
    Data, Expression,
};

/// +, takes an arbitrary number of arguments and sums them up. If just one argument is passed, it
/// will be cast to a number. Returns `Value::Null` if one argument cannot be coerced into a
/// number.
pub fn compute(args: &[Expression], data: &Data) -> Value {
    let a = match args.get(0) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };

    let b = match args.get(1) {
        Some(arg) => arg.compute(data),
        None => return Value::Bool(false),
    };
    if let Some(val) = logic::before(&a, &b) {
        Value::Bool(val)
    } else {
        Value::Null
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use serde_json::json;

     #[test]
    fn test() {
        assert_eq!(json!(false), compute_const!(json!("2020-01-02"), json!("2020-01-01")));
        assert_eq!(json!(false), compute_const!(json!("2020-01-02"), json!("2020-01-02")));
        assert_eq!(json!(true), compute_const!(json!("2020-01-01"), json!("2020-01-02")));
    }
}

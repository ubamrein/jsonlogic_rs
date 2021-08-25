use std::ops::Add;

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
    if args.len() != 3 {
        return Value::Null;
    }
    let val = &args[0].compute(data);
    let date_string = logic::parse_date_with_offset(val)
        .or_else(|| {
            logic::parse_date_without_offset(val).map(|dt| {
                chrono::offset::Utc
                    .fix()
                    .from_local_datetime(&dt)
                    .earliest()
                    .unwrap()
            })
        })
        .or_else(|| {
            logic::parse_date_without_time(val).map(|dt| {
                chrono::offset::Utc
                    .fix()
                    .from_local_date(&dt)
                    .earliest()
                    .unwrap()
                    .and_hms(0, 0, 0)
            })
        });
    let date = if let Some(date_string) = date_string {
        date_string
    } else {
        return Value::Null;
    };
    let offset = if let Some(a) = parse_float(&args[1].compute(data)) {
        a as i64
    } else {
        return Value::Null;
    };
    let unit = match args[2].compute(data) {
        Value::String(s) => s,
        _ => return Value::Null,
    };

    let date = match unit.as_str() {
        "year" | "years" => date.add(chrono::Duration::weeks(offset * 52)),
        "month" | "months" => date.add(chrono::Duration::weeks(offset * 4)),
        "week" | "weeks" => date.add(Duration::weeks(offset)),
        "day" | "days" => date.add(Duration::days(offset)),
        "hour" | "hours" => date.add(Duration::hours(offset)),
        "minute" | "minutes" => date.add(Duration::minutes(offset)),
        "second" | "seconds" => date.add(Duration::seconds(offset)),
        _ => return Value::Null,
    };

    Value::String(date.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::compute_const;
    use chrono::{DateTime, FixedOffset, Local, NaiveDate, NaiveDateTime};
    use serde_json::json;

    #[test]
    fn test() {
        assert_eq!(compute_const!(), json!(Value::Null));
        assert_eq!(compute_const!(Value::Null), Value::Null);

        let timestamp = "2021-01-01T00:00:00Z"
            .parse::<DateTime<FixedOffset>>()
            .unwrap()
            .timestamp_millis();

        assert_eq!(
            compute_const!(
                Value::String("2021-01-01T00:00:00.000Z".to_string()),
                Value::Number(0.into()),
                Value::String("day".to_string())
            ),
            timestamp
        );

        let timestamp = "2021-01-01T00:00:00"
            .parse::<NaiveDateTime>()
            .unwrap()
            .timestamp_millis();
        assert_eq!(
            compute_const!(
                Value::String("2021-01-01T00:00:00.000".to_string()),
                Value::Number(0.into()),
                Value::String("day".to_string())
            ),
            timestamp
        );

        let timestamp = "2021-01-01"
            .parse::<NaiveDate>()
            .unwrap()
            .and_hms(0,0,0)
            .timestamp_millis();
           
        assert_eq!(
            compute_const!(
                Value::String("2021-01-01".to_string()),
                Value::Number(0.into()),
                Value::String("day".to_string())
            ),
            timestamp
        );
    }

    #[test]
    fn test_addition() {
         let timestamp = "2021-01-02T00:00:00"
            .parse::<NaiveDateTime>()
            .unwrap()
            .timestamp_millis();
        assert_eq!(
            compute_const!(
                Value::String("2021-01-01T00:00:00.000".to_string()),
                Value::Number(1.into()),
                Value::String("day".to_string())
            ),
            timestamp
        );


         let timestamp = "2021-01-01T01:00:00Z"
            .parse::<DateTime<FixedOffset>>()
            .unwrap()
            .timestamp_millis();

        assert_eq!(
            compute_const!(
                Value::String("2021-01-01T00:00:00.000Z".to_string()),
                Value::Number(1.into()),
                Value::String("hours".to_string())
            ),
            timestamp
        );
    }
}

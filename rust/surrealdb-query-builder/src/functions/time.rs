/*
 * Author: Oyelowo Oyedayo
 * Email: oyelowooyedayo@gmail.com
 * Copyright (c) 2023 Oyelowo Oyedayo
 * Licensed under the MIT license
 */

// Time functions
// These functions can be used when working with and manipulating datetime values.
//
// Function	Description
// time::day()	Extracts the day as a number from a datetime
// time::floor()	Rounds a datetime down by a specific duration
// time::group()	Groups a datetime by a particular time interval
// time::hour()	Extracts the hour as a number from a datetime
// time::mins()	Extracts the minutes as a number from a datetime
// time::month()	Extracts the month as a number from a datetime
// time::nano()	Returns the number of nanoseconds since the UNIX epoch
// time::now()	Returns the current datetime
// time::round()	Rounds a datetime up by a specific duration
// time::secs()	Extracts the secs as a number from a datetime
// time::unix()	Returns the number of seconds since the UNIX epoch
// time::wday()	Extracts the week day as a number from a datetime
// time::week()	Extracts the week as a number from a datetime
// time::yday()	Extracts the yday as a number from a datetime
// time::year()	Extracts the year as a number from a datetime

use crate::{
    sql::{Binding, Buildable, ToRawStatement},
    Field,
};

use super::array::Function;
use surrealdb::sql;

pub struct Datetime(sql::Value);

impl From<Datetime> for sql::Value {
    fn from(value: Datetime) -> Self {
        value.0
    }
}

impl<T: Into<sql::Datetime>> From<T> for Datetime {
    fn from(value: T) -> Self {
        let value: sql::Datetime = value.into();
        Self(value.into())
    }
}

impl From<Field> for Datetime {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}

pub struct Duration(sql::Value);

impl From<Duration> for sql::Value {
    fn from(value: Duration) -> Self {
        value.0
    }
}

impl<T: Into<sql::Duration>> From<T> for Duration {
    fn from(value: T) -> Self {
        let value: sql::Duration = value.into();
        Self(value.into())
    }
}

impl From<Field> for Duration {
    fn from(value: Field) -> Self {
        Self(value.into())
    }
}
fn day_fn(datetime: impl Into<Datetime>) -> Function {
    let binding = Binding::new(datetime.into());
    let query_string = format!("time::day({})", binding.get_param_dollarised());

    Function {
        query_string,
        bindings: vec![binding],
    }
}

#[macro_export]
macro_rules! day {
    ( $datetime:expr ) => {
        crate::functions::time::day_fn($datetime)
    };
}

pub use day;

fn floor_fn(datetime: impl Into<Datetime>, duration: impl Into<Duration>) -> Function {
    let datetime_binding = Binding::new(datetime.into());
    let duration_binding = Binding::new(duration.into());
    let query_string = format!(
        "time::floor({}, {})",
        datetime_binding.get_param_dollarised(),
        duration_binding.get_param_dollarised()
    );

    Function {
        query_string,
        bindings: vec![datetime_binding, duration_binding],
    }
}

#[macro_export]
macro_rules! floor {
    ( $datetime:expr, $duration:expr ) => {
        crate::functions::time::floor_fn($datetime, $duration)
    };
}

pub use floor;

#[test]
fn test_day_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let result = day!(rebirth_date);

    assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
    assert_eq!(result.to_raw().to_string(), "time::day(rebirth_date)");
}
#[test]
fn test_day_macro_with_plain_datetime() {
    let dt = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    );
    let result = day!(dt);
    assert_eq!(result.fine_tune_params(), "time::day($_param_00000001)");
    assert_eq!(
        result.to_raw().to_string(),
        "time::day('1970-01-01T00:01:01Z')"
    );
}

#[test]
fn test_floor_macro_with_datetime_field() {
    let rebirth_date = Field::new("rebirth_date");
    let duration = Field::new("duration");
    let result = floor!(rebirth_date, duration);

    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "time::floor(rebirth_date, duration)"
    );
}

#[test]
fn test_floor_macro_with_plain_datetime_and_duration() {
    let dt = chrono::DateTime::<chrono::Utc>::from_utc(
        chrono::NaiveDateTime::from_timestamp(61, 0),
        chrono::Utc,
    );
    let duration = std::time::Duration::from_secs(24 * 60 * 60 * 7);
    let result = floor!(dt, duration);
    assert_eq!(
        result.fine_tune_params(),
        "time::floor($_param_00000001, $_param_00000002)"
    );
    assert_eq!(
        result.to_raw().to_string(),
        "time::floor('1970-01-01T00:01:01Z', 1w)"
    );
}

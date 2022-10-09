use parse_duration::parse;
use parse_duration::parse::Error;
use std::num::ParseIntError;
use std::time::Duration;

pub fn convert_udf_time_to_sec(udf_time: &str) -> Option<i64> {
    match udf_time.parse::<i64>() {
        Ok(_) => match parse(udf_time) {
            Ok(time) => Some((time.as_secs() * 60) as i64),
            Err(_) => None,
        },
        Err(_) => match parse(udf_time) {
            Ok(time) => Some((time.as_secs()) as i64),
            Err(_) => None,
        },
    }
}

use std::num::ParseIntError;
use std::time::Duration;

pub fn convert_udf_time_to_minute(udf_time: Option<String>) -> Option<i64> {
    let udf_time = udf_time?;
    let time: &str = &udf_time[0..udf_time.len() - 1];
    let multiplier: i64 = match udf_time.chars().last()? {
        's' => 1,
        'm' => 60,
        'h' => 60 * 60,
        'D' => 24 * 60 * 60,
        'W' => 7 * 24 * 60 * 60,
        'M' => 30 * 24 * 60 * 60,
        'Y' => 365 * 24 * 60 * 60,
        _ => 60,
    };
    Some(time.parse::<i64>().unwrap_or(1) * multiplier)
}

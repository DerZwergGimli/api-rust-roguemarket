use std::num::ParseIntError;
use std::time::Duration;

pub fn convert_udf_time_to_minute(udf_time: &str) -> Option<i64> {
    if (udf_time.contains("D")) {
        let time: &str = &udf_time[0..udf_time.len() - 1];
        return Some(time.parse::<i64>().unwrap_or(60) * 60 * 24);
    } else if (udf_time.contains("W")) {
        let time: &str = &udf_time[0..udf_time.len() - 1];
        return Some(time.parse::<i64>().unwrap_or(60) * 60 * 24 * 7);
    } else if (udf_time.contains("M")) {
        let time: &str = &udf_time[0..udf_time.len() - 1];
        return Some(time.parse::<i64>().unwrap_or(60) * 60 * 24 * 7 * 4);
    } else {
        return Some(udf_time.parse::<i64>().unwrap_or(60));
    }
    None
}

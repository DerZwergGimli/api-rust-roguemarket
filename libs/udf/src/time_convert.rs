use std::num::ParseIntError;
use std::time::Duration;
use std::str;

pub fn convert_udf_time_to_seconds(input: Option<String>) -> Option<i64> {
    if let Some(input_str) = input {
        let trimmed_str = input_str.trim();
        let last_char = trimmed_str.chars().last()?;

        match last_char {
            's' => trimmed_str[..trimmed_str.len() - 1].parse().ok(),
            'm' => trimmed_str[..trimmed_str.len() - 1].parse::<i64>().map(|x| x * 60).ok(),
            'h' => trimmed_str[..trimmed_str.len() - 1].parse::<i64>().map(|x| x * 3600).ok(),
            'D' => trimmed_str[..trimmed_str.len() - 1].parse::<i64>().map(|x| x * 86400).ok(),
            'W' => trimmed_str[..trimmed_str.len() - 1].parse::<i64>().map(|x| x * 604800).ok(),
            'M' => trimmed_str[..trimmed_str.len() - 1].parse::<i64>().map(|x| x * 2592000).ok(), //30 days
            _ => trimmed_str.parse::<i64>().map(|x| x * 60).ok().or(Some(0)),
        }
    } else {
        None
    }
}


pub fn convert_udf_time_to_timestamp_minute(input: Option<String>) -> Option<i64> {
    if let Some(time_str) = input {
        if let Some(time) = time_str.strip_suffix('s') {
            if let Ok(num) = time.parse::<i64>() {
                return Some(num);
            }
        } else if let Ok(num) = time_str.parse::<i64>() {
            return Some(num);
        } else if let Some(time) = time_str.strip_suffix('D') {
            if let Ok(num) = time.parse::<i64>() {
                return Some(num * 24 * 60);
            }
        } else if let Some(time) = time_str.strip_suffix('W') {
            if let Ok(num) = time.parse::<i64>() {
                return Some(num * 7 * 24 * 60);
            }
        }
    }

    None
}




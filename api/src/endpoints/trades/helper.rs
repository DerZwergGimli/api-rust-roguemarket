use std::collections::HashMap;

use chrono::NaiveDateTime;
use serde::Serialize;

use database_psql::model::Trade;

pub enum VolumeInterval {
    Daily,
    Hourly,
    Monthly,
}

#[derive(Debug, Serialize)]
pub struct VolumeData {
    time: NaiveDateTime,
    volume: f64,
}

pub fn calculate_volume(trades: Vec<Trade>, interval: VolumeInterval) -> Vec<VolumeData> {
    let mut volume_by_interval = HashMap::new();

    for trade in trades {
        let timestamp = trade.timestamp_ts.timestamp();
        let interval_start = match interval {
            VolumeInterval::Daily => NaiveDateTime::from_timestamp(timestamp / 86400 * 86400, 0),
            VolumeInterval::Hourly => NaiveDateTime::from_timestamp(timestamp / 3600 * 3600, 0),
            VolumeInterval::Monthly => NaiveDateTime::from_timestamp(timestamp / 2628000 * 2628000, 0),
        };
        let interval_key = interval_start.format("%Y-%m-%d %H:%M:%S").to_string();

        let volume = volume_by_interval.entry(interval_key).or_insert(0.0);
        *volume += trade.total_cost;
    }

    let mut volume_data = Vec::new();
    for (interval_key, volume) in volume_by_interval {
        let time = NaiveDateTime::parse_from_str(&interval_key, "%Y-%m-%d %H:%M:%S").unwrap();
        let volume_datum = VolumeData {
            time,
            volume,
        };
        volume_data.push(volume_datum);
    }

    volume_data
}
use database_psql::model::Trade;
use crate::endpoints::Data;
use crate::endpoints::udf::udf_history_t::UdfHistory;

struct OHLC {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn ohlc_converter(data: &[Trade], resolution_secounds: Option<i64>) -> UdfHistory {
    let mut result = UdfHistory {
        s: "ok".to_string(),
        t: vec![],
        c: vec![],
        o: vec![],
        h: vec![],
        l: vec![],
        v: vec![],
    };

    let mut current_ohlc: Option<OHLC> = None;

    for item in data {
        if let Some(ohlc) = &mut current_ohlc {
            if item.timestamp >= ohlc.timestamp + resolution_secounds.unwrap_or(60) { // new minute
                result.t.push(ohlc.timestamp);
                result.c.push(ohlc.close);
                result.o.push(ohlc.open);
                result.h.push(ohlc.high);
                result.l.push(ohlc.low);
                result.v.push(ohlc.volume); // volume not supported
                *ohlc = OHLC {
                    timestamp: item.timestamp,
                    open: item.total_cost,
                    high: item.total_cost,
                    low: item.total_cost,
                    close: item.total_cost,
                    volume: item.asset_change as f64,
                };
            } else {
                ohlc.high = ohlc.high.max(item.total_cost);
                ohlc.low = ohlc.low.min(item.total_cost);
                ohlc.close = item.total_cost;
            }
        } else {
            current_ohlc = Some(OHLC {
                timestamp: item.timestamp,
                open: item.total_cost,
                high: item.total_cost,
                low: item.total_cost,
                close: item.total_cost,
                volume: item.asset_change as f64,
            });
        }
    }

    if let Some(ohlc) = current_ohlc {
        result.t.push(ohlc.timestamp);
        result.c.push(ohlc.close);
        result.o.push(ohlc.open);
        result.h.push(ohlc.high);
        result.l.push(ohlc.low);
        result.v.push(ohlc.volume); // volume not supported
    }

    result
}
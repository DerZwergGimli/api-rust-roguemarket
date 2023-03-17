use database_psql::model::Trade;

use crate::endpoints::udf::udf_history_t::UdfHistory;

struct OHLC {
    timestamp: i64,
    open: f64,
    high: f64,
    low: f64,
    close: f64,
    volume: f64,
}

pub fn ohlc_converter(data: &[Trade], timeframe_seconds: Option<i64>) -> UdfHistory {
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
            if item.timestamp >= (ohlc.timestamp) + timeframe_seconds.unwrap_or(60) {
                result.t.push(ohlc.timestamp);
                result.c.push(ohlc.close);
                result.o.push(ohlc.open);
                result.h.push(ohlc.high);
                result.l.push(ohlc.low);
                result.v.push(ohlc.volume);
                *ohlc = OHLC {
                    timestamp: item.timestamp,
                    open: item.price,
                    high: item.price,
                    low: item.price,
                    close: item.price,
                    volume: item.asset_change as f64,
                };
            } else {
                ohlc.high = ohlc.high.max(item.price);
                ohlc.low = ohlc.low.min(item.price);
                ohlc.close = item.price;
            }
        } else {
            current_ohlc = Some(OHLC {
                timestamp: item.timestamp,
                open: item.price,
                high: item.price,
                low: item.price,
                close: item.price,
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

fn calc_price(item: &Trade) -> f64 {
    item.total_cost / item.asset_change as f64
}
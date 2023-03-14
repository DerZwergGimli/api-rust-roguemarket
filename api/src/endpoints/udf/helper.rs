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

pub fn ohlc_converter(data: &[Trade], resolution_minute: Option<i64>) -> UdfHistory {
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
            if item.timestamp >= ohlc.timestamp + resolution_minute.unwrap_or(60) {
                result.t.push(ohlc.timestamp);
                result.c.push(ohlc.close);
                result.o.push(ohlc.open);
                result.h.push(ohlc.high);
                result.l.push(ohlc.low);
                result.v.push(ohlc.volume);
                *ohlc = OHLC {
                    timestamp: item.timestamp,
                    open: calc_price(item),
                    high: calc_price(item),
                    low: calc_price(item),
                    close: calc_price(item),
                    volume: item.asset_change as f64,
                };
            } else {
                ohlc.high = ohlc.high.max(calc_price(item));
                ohlc.low = ohlc.low.min(calc_price(item));
                ohlc.close = calc_price(item);
            }
        } else {
            current_ohlc = Some(OHLC {
                timestamp: item.timestamp,
                open: calc_price(item),
                high: calc_price(item),
                low: calc_price(item),
                close: calc_price(item),
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
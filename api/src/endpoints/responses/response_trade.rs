use std::convert::Infallible;

use log::warn;
use serde::Serialize;
use warp::Reply;

use database_psql::model::Trade;

use crate::endpoints::responses::response_error::ResponseError;

pub fn create_response<T: Serialize>(cursor_db: &Vec<T>) -> Result<impl Reply, Infallible> {
    return if cursor_db.is_empty() {
        warn!("There seems to be no data...");
        Ok(warp::reply::json(&ResponseError {
            s: 1,
            errmsg: "No data found".to_string(),
        }))
    } else {
        Ok(warp::reply::json(&cursor_db))
    };
}

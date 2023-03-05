use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct UdfError {
    pub(crate) s: Status,
    //pub(crate) errmsg: Option<String>,
    pub(crate) nextTime: Option<i64>,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub enum Status {
    ok,
    error,
    no_data,
}

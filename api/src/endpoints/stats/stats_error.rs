use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Debug, Serialize, Deserialize, ToSchema, Clone)]
pub struct StatsError {
    pub(crate) s: i64,
    pub(crate) errmsg: String,
}
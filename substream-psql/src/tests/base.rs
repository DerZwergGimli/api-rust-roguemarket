#[cfg(test)]
mod tests {
    use std::env;
    use std::sync::Arc;

    use assert_json_diff::assert_json_include;
    use futures::StreamExt;
    use log::{error, warn};
    use serde_json::json;
    use structopt::StructOpt;

    use crate::{Config, read_package};
    use crate::helper::{extract_database_changes_from_map, map_trade_to_struct, request_token};
    use crate::pb::database::DatabaseChanges;
    use crate::pb::database::table_change::Operation;
    use crate::substreams::SubstreamsEndpoint;
    use crate::substreams_stream::{BlockResponse, SubstreamsStream};
    use crate::tests::test_helper_substreams;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }

    #[tokio::test]
    async fn test_map001() {
        let expected_data = "[Field { name: \"signature\", new_value: \"5UEwenEizP8qiAGZ1ZqpgzXHfwT24KEuR1G4gBzCiz5EcPFsJ3MQQS1xdASaypK3y5RgbaMBmbn3jWBoJcgBHHPH\", old_value: \"5UEwenEizP8qiAGZ1ZqpgzXHfwT24KEuR1G4gBzCiz5EcPFsJ3MQQS1xdASaypK3y5RgbaMBmbn3jWBoJcgBHHPH\" }, Field { name: \"block\", new_value: \"143203232\", old_value: \"143203232\" }, Field { name: \"timestamp\", new_value: \"1658823438\", old_value: \"1658823438\" }, Field { name: \"order_taker\", new_value: \"FnuG4ZMtLvassRvFmmSuH3HKBdfVVWkfrA3GtJrfeQax\", old_value: \"FnuG4ZMtLvassRvFmmSuH3HKBdfVVWkfrA3GtJrfeQax\" }, Field { name: \"currency_mint\", new_value: \"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v\", old_value: \"EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v\" }, Field { name: \"asset_mint\", new_value: \"ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK\", old_value: \"ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK\" }, Field { name: \"order_initializer\", new_value: \"9Bu7dde6pJtSpgUzcZxKbnXK1qoPbyi7ANDxw62rwUjB\", old_value: \"9Bu7dde6pJtSpgUzcZxKbnXK1qoPbyi7ANDxw62rwUjB\" }, Field { name: \"asset_change\", new_value: \"15000\", old_value: \"15000\" }, Field { name: \"market_fee\", new_value: \"0\", old_value: \"0\" }, Field { name: \"total_cost\", new_value: \"0.525\", old_value: \"0.525\" }, Field { name: \"price\", new_value: \"0.525\", old_value: \"0.525\" }]";
        test_helper_substreams(expected_data.to_string(), 143203232, 143203233);
    }
}
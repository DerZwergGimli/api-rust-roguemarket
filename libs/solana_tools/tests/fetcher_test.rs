#[cfg(test)]
mod fetcher_test {
    use assert_json_diff::assert_json_eq;
    use helper::filehelper::{read_file, write_file};
    use solana_client::rpc_client::RpcClient;
    use solana_client::rpc_response::RpcConfirmedTransactionStatusWithSignature;
    use solana_tools::fetcher::fetcher::Fetcher;
    use solana_transaction_status::EncodedConfirmedTransactionWithStatusMeta;
    use staratlas::symbolstore::SymbolStore;
    use types::databasetrade::DBTrade;

    #[test]
    fn get_signatures() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        let signatures =
            fetcher.fetch_signatures("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg", Some(10));
        assert_eq!(signatures.len(), 10);
    }

    #[test]
    fn get_transactions() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        let signatures =
            fetcher.fetch_signatures("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg", Some(10));
        let transactions = fetcher.fetch_transactions(&signatures);
        assert_eq!(transactions.len(), 10);
    }

    #[test]
    fn filter_transactions() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        let signatures = helper::filehelper::read_file::<
            Vec<RpcConfirmedTransactionStatusWithSignature>,
        >("tests/samples/signatures.sample.json");

        let transactions = fetcher.fetch_transactions(&signatures);
        let transactions_length = &transactions.len();
        let filtered_transactions = fetcher.filter_transactions_for_exchange(transactions);

        let mut less = false;
        if transactions_length > &filtered_transactions.len() {
            less = true
        }

        assert_eq!(less, true);
    }

    #[test]
    fn map_transaction_001() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        //Load sample file INPUT
        let filtered_transactions = read_file::<Vec<EncodedConfirmedTransactionWithStatusMeta>>(
            "tests/samples/input/tx_001.in.sample.json",
        );

        let mapped = fetcher.map_transactions(&filtered_transactions);

        //Load sample file OUTPUT
        let expected = read_file::<Vec<DBTrade>>("tests/samples/output/db_001.out.sample.json");

        assert_json_eq!(mapped, expected);
    }

    #[test]
    fn map_transaction_002() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        //Load sample file INPUT
        let signatures = read_file::<Vec<RpcConfirmedTransactionStatusWithSignature>>(
            "tests/samples/input/sig_002.in.sample.json",
        );

        let transactions = fetcher.fetch_transactions(&signatures);
        let filtered_transactions = fetcher.filter_transactions_for_exchange(transactions);

        let mapped = fetcher.map_transactions(&filtered_transactions);

        //Load sample file OUTPUT
        let expected = read_file::<Vec<DBTrade>>("tests/samples/output/db_002.out.sample.json");

        assert_json_eq!(mapped, expected);
    }
    #[test]
    fn map_transaction_003() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        //Load sample file INPUT
        let signatures = read_file::<Vec<RpcConfirmedTransactionStatusWithSignature>>(
            "tests/samples/input/sig_003.in.sample.json",
        );
        let transactions = fetcher.fetch_transactions(&signatures);
        let filtered_transactions = fetcher.filter_transactions_for_exchange(transactions);
        let mapped = fetcher.map_transactions(&filtered_transactions);

        //Load sample file OUTPUT
        let expected = read_file::<Vec<DBTrade>>("tests/samples/output/db_003.out.sample.json");

        assert_json_eq!(mapped, expected);
    }

    #[test]
    fn map_transactions() {
        let store = read_file::<SymbolStore>("tests/samples/input/store.sample.json");
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string()),
            store: store,
        };

        let filtered_transactions = read_file::<Vec<EncodedConfirmedTransactionWithStatusMeta>>(
            "tests/samples/input/transactions-filtered.in.sample.json",
        );

        let mapped = fetcher.map_transactions(&filtered_transactions);
        let expected =
            read_file::<Vec<DBTrade>>("tests/samples/output/transactions-filtered.out.sample.json");

        assert_json_eq!(mapped, expected);
    }
}

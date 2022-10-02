#[cfg(test)]
mod fetcher_test {
    use std::borrow::Borrow;

    use solana_client::rpc_client::RpcClient;

    use solana_tools::fetcher::fetcher::Fetcher;

    #[test]
    fn get_signatures() {
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string())
        };

        let signatures = fetcher.fetch_signatures("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg", Some(10));
        assert_eq!(signatures.len(), 10);
    }

    #[test]
    fn get_transactions() {
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string())
        };

        let signatures = fetcher.fetch_signatures("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg", Some(10));
        let transactions = fetcher.fetch_transactions(&signatures);
        assert_eq!(transactions.len(), 10);
    }

    #[test]
    fn filter_transactions() {
        let fetcher = Fetcher {
            client: RpcClient::new("https://ssc-dao.genesysgo.net/".to_string())
        };

        let signatures = fetcher.fetch_signatures("traderDnaR5w6Tcoi3NFm53i48FTDNbGjBSZwWXDRrg", Some(100));
        let mut transactions = fetcher.fetch_transactions(&signatures);
        let tmp = &transactions;
        let filtered_transactions = fetcher.filter_transactions_forExchange(&transactions);

        let mut less = false;
        if (&transactions.len() > &filtered_transactions.len()) {
            less = true
        }

        assert_eq!(less, true);
    }

    #[test]
    fn map_transactions() {
        assert_eq!(false, true);
    }
}
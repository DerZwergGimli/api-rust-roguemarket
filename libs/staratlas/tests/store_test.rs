#[cfg(test)]
mod store_test {
    use helper::filehelper::write_file;
    use staratlas::symbolstore::BuilderSymbolStore;
    #[tokio::test]
    async fn init_store() {
        let symbolStore = BuilderSymbolStore::new();
        let store = symbolStore.init().await;

        write_file("tests/store.sample.json", &store);

        assert_eq!(store.assets.len() > 10, true);
        assert_eq!(store.currencies.len(), 2)
    }
}

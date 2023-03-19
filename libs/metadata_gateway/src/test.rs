use mpl_token_metadata::state::Metadata;

#[cfg(test)]
mod tests {
    use std::fs::metadata;
    use std::str::FromStr;

    use mpl_token_metadata::solana_program::pubkey::Pubkey;
    use mpl_token_metadata::state::AuthorityType::Metadata;
    use solana_client::rpc_client::RpcClient;

    use crate::{get_metadata, parse_symbol, request_metadata_symbol};

    #[test]
    fn is_AMMO() {
        let symbol = request_metadata_symbol(
            "https://api.mainnet-beta.solana.com".to_string(),
            "ammoK8AkX2wnebQb35cDAZtTkvsXQbi82cGeTnUvvfK".to_string());

        assert_eq!(symbol, "AMMO");
    }

    #[test]
    fn is_LSTAND() {
        let symbol = request_metadata_symbol(
            "https://api.mainnet-beta.solana.com".to_string(),
            "DB8CSxoakPRtXhHcc2cA3iETWfGaYY6zE2T8huJTE2Nw".to_string());

        assert_eq!(symbol, "LSTAND");
    }


    #[test]
    fn is_USDC() {
        let symbol = request_metadata_symbol(
            "https://api.mainnet-beta.solana.com".to_string(),
            "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v".to_string());

        assert_eq!(symbol, "USDC");
    }
}



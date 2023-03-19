use std::str::FromStr;

use borsh::de::BorshDeserialize;
use mpl_token_metadata::solana_program::pubkey::Pubkey;
use mpl_token_metadata::state::Metadata;
use solana_client::rpc_client::RpcClient;

mod test;


pub fn request_metadata_symbol(rpc: String, mint: String) -> String {
    let client = RpcClient::new(rpc);
    let mint = Pubkey::from_str(mint.as_str()).unwrap();

    let metadata = get_metadata(&client, &mint).unwrap();
    let symbol = parse_symbol(metadata);

    symbol
}

fn get_metadata(
    rpc: &RpcClient,
    mint_address: &Pubkey,
) -> Result<Metadata, Box<dyn std::error::Error>> {
    let (meta_addr, _) = mpl_token_metadata::pda::find_metadata_account(&mint_address);
    let metadata_account = rpc.get_account(&meta_addr).unwrap();
    let acct = &mut &metadata_account.data[..];

    Metadata::deserialize(acct).map_err(|e| e.into())
}

fn parse_symbol(metadata: Metadata) -> String {
    let symbol_str = metadata.data.symbol.trim_matches(char::from(0));
    symbol_str.to_string()
}
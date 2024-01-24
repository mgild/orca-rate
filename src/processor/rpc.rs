use anchor_lang::AccountDeserialize;
use anyhow::Ok;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::pubkey::Pubkey;
use whirlpool::state::{TickArray, Whirlpool};

pub async fn fetch_whirlpool_account(
    rpc_client: &RpcClient,
    pda: Pubkey,
) -> anyhow::Result<Whirlpool> {
    let account = rpc_client.get_account(&pda).await?;
    let whirlpool = Whirlpool::try_deserialize(&mut account.data.as_slice()).unwrap();
    Ok(whirlpool)
}

pub async fn fetch_tick_arrays(
    rpc_client: &RpcClient,
    keys: Vec<Pubkey>,
) -> anyhow::Result<Vec<TickArray>> {
    let accounts = rpc_client.get_multiple_accounts(&keys).await?;

    let mut tick_arrays: Vec<TickArray> = vec![];
    accounts.iter().for_each(|account| {
        let tick_array =
            TickArray::try_deserialize(&mut account.clone().unwrap().data.as_slice()).unwrap();
        tick_arrays.push(tick_array);
    });

    Ok(tick_arrays)
}

pub async fn fetch_mint_decimals(rpc_client: &RpcClient, mint: Pubkey) -> anyhow::Result<u8> {
    let data = rpc_client.get_account_data(&mint).await?;
    Ok(data[44])
}

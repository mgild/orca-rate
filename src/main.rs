use std::env;

mod constants;
mod processor;
mod structs;
mod utils;

use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::{constants::*, processor::*, utils::*};

#[tokio::main]
async fn main() {
    // Environment configuration
    dotenv::dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Suppose to get quote from SOL to USDC as 1 SOL
    let mint_in = get_pubkey(SOL_MINT);
    let mint_out = get_pubkey(USDC_MINT);
    let input_amount: f64 = 2.5;
    println!("Input token: {}", mint_in);
    println!("Input amount: {}", input_amount);
    println!("Output token: {}", mint_out);

    let output_amount = get_whirlpool_quote(&rpc_client, mint_in, mint_out, input_amount).await;
    println!("Whirlpool expected amount: {}", output_amount);

    let output_amount = get_legacy_quote(
        &rpc_client,
        mint_in,
        mint_out,
        get_pubkey(SOL_USDC_TOKEN_A_DEPOSIT),
        get_pubkey(SOL_USDC_TOKEN_B_DEPOSIT),
        input_amount,
    )
    .await;
    println!("Legacypool expected amount: {}", output_amount);
}

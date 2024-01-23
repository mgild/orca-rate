use std::{env, ops::Div};

mod constants;
mod processor;
mod utils;

use num_traits::{pow, FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_sdk::commitment_config::CommitmentConfig;

use crate::{constants::*, processor::*, utils::*};

#[tokio::main]
async fn main() {
    // Environment configuration
    dotenv::dotenv().ok();
    let rpc_url = env::var("RPC_URL").expect("RPC_URL not set in .env");

    // Suppose to get quote from SOL to USDC as 1 SOL
    let mint_in = get_pubkey(SOL_MINT);
    let decimals_in = Decimal::from(pow(10, SOL_DECIMALS));
    let mint_out = get_pubkey(BONK_MINT);
    let decimals_out = Decimal::from(pow(10, BONK_DECIMALS));
    let input_amount: f64 = 1.0;
    println!("Input token: {}", mint_in);
    println!("Input amount: {}", input_amount);
    println!("Output token: {}", mint_out);

    // Initialize variables
    let orca_whirepool_id = get_pubkey(ORCA_WHIRLPOOL_ID);
    let whirlpool_config_id = get_pubkey(WHIRLPOOL_CONFIG_ID);
    let rpc_client = RpcClient::new_with_commitment(rpc_url, CommitmentConfig::confirmed());

    // Get whirepool pda from token pair
    let whirlpool_pda = get_whirlpool_pda(
        orca_whirepool_id,
        whirlpool_config_id,
        mint_in,
        mint_out,
        TICK_SPACE,
    );

    // Fetch whirepool accounts
    let whirlpool = parse_whirlpool_account(&rpc_client, whirlpool_pda)
        .await
        .expect("Whirlpool account not valid");

    // Determine swap direction
    let a_to_b = whirlpool.token_mint_a.eq(&mint_in);

    // Fetch tick array
    let tick_pdas = build_tick_array_pdas(
        whirlpool.tick_current_index,
        whirlpool.tick_spacing as i32,
        a_to_b,
        orca_whirepool_id,
        whirlpool_pda,
    );
    let tick_arrays = parse_tick_arrays(&rpc_client, tick_pdas)
        .await
        .expect("Tick array accounts not valid");

    // Get quote
    let amount_in = (Decimal::from_f64(input_amount).unwrap_or_default() * decimals_in)
        .to_u64()
        .unwrap_or_default();
    let [amount_a, amount_b] = get_swap_quote(&whirlpool, tick_arrays, amount_in, true, a_to_b);
    let amount_out = if a_to_b { amount_b } else { amount_a };
    let output_amount = Decimal::from(amount_out).div(decimals_out);
    println!("Output amount: {}", output_amount);
}

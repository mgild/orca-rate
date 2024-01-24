use num_traits::{pow, FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;

use crate::{fetch_mint_decimals, ORCA_SWAP_FEE};

pub async fn get_legacy_quote(
    rpc_client: &RpcClient,
    mint_in: Pubkey,
    mint_out: Pubkey,
    token_a: Pubkey,
    token_b: Pubkey,
    input_amount: f64,
) -> f64 {
    // Get decimals for in & out token
    let decimals_in = fetch_mint_decimals(&rpc_client, mint_in)
        .await
        .expect("Invalid input token");
    let decimals_in_pow = Decimal::from(pow(10, decimals_in as usize));
    let decimals_out = fetch_mint_decimals(&rpc_client, mint_out)
        .await
        .expect("Invalid input token");
    let decimals_out_pow = Decimal::from(pow(10, decimals_out as usize));

    let amount_in = (Decimal::from_f64(input_amount).unwrap_or_default() * decimals_in_pow)
        .to_u64()
        .unwrap_or_default();

    // Get legacy pool balance
    let token_a_balance = rpc_client
        .get_token_account_balance(&token_a)
        .await
        .unwrap()
        .amount
        .parse::<u64>()
        .unwrap();
    let token_b_balance = rpc_client
        .get_token_account_balance(&token_b)
        .await
        .unwrap()
        .amount
        .parse::<u64>()
        .unwrap();

    let amount_out = get_expected_output_amount(token_a_balance, token_b_balance, amount_in);
    let output_amount = Decimal::from(amount_out) / decimals_out_pow;
    output_amount.to_f64().unwrap_or_default()
}

fn get_expected_output_amount(
    input_pool_balance: u64,
    output_pool_balance: u64,
    input_amount: u64,
) -> u64 {
    let ib: u128 = From::from(input_pool_balance);
    let ob: u128 = From::from(output_pool_balance);
    let ia: u128 = From::from(input_amount);

    let orca_fee = ORCA_SWAP_FEE;

    let of = div_ceiling(ia * orca_fee.0, orca_fee.1);
    let mia = ia - of;

    let invariant = ib * ob;
    let next_ib = ib + mia;
    let next_ob = div_ceiling(invariant, next_ib);

    let expected_oa = ob - next_ob;

    let expected_oa_u64: u64 = TryFrom::try_from(expected_oa).unwrap();
    expected_oa_u64
}

fn div_ceiling(numerator: u128, denominator: u128) -> u128 {
    if numerator % denominator == 0 {
        numerator / denominator
    } else {
        numerator / denominator + 1
    }
}

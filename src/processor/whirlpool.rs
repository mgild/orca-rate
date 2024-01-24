use num_traits::{pow, FromPrimitive, ToPrimitive};
use rust_decimal::Decimal;
use solana_client::nonblocking::rpc_client::RpcClient;
use solana_program::pubkey::Pubkey;
use std::cell::RefCell;
use whirlpool::manager::swap_manager::swap;
use whirlpool::state::{TickArray, Whirlpool};
use whirlpool::util::SwapTickSequence;

use crate::{
    build_tick_array_pdas, fetch_mint_decimals, fetch_tick_arrays, fetch_whirlpool_account,
    get_pubkey, get_whirlpool_pda, MAX_SQRT_PRICE, MIN_SQRT_PRICE, ORCA_WHIRLPOOL_ID, TICK_SPACE,
    WHIRLPOOL_CONFIG_ID,
};

pub async fn get_whirlpool_quote(
    rpc_client: &RpcClient,
    mint_in: Pubkey,
    mint_out: Pubkey,
    input_amount: f64,
) -> f64 {
    let orca_whirepool_id = get_pubkey(ORCA_WHIRLPOOL_ID);
    let whirlpool_config_id = get_pubkey(WHIRLPOOL_CONFIG_ID);

    // Get decimals for in & out token
    let decimals_in = fetch_mint_decimals(&rpc_client, mint_in)
        .await
        .expect("Invalid input token");
    let decimals_in_pow = Decimal::from(pow(10, decimals_in as usize));
    let decimals_out = fetch_mint_decimals(&rpc_client, mint_out)
        .await
        .expect("Invalid input token");
    let decimals_out_pow = Decimal::from(pow(10, decimals_out as usize));

    // Get whirepool pda from token pair
    let whirlpool_pda = get_whirlpool_pda(
        orca_whirepool_id,
        whirlpool_config_id,
        mint_in,
        mint_out,
        TICK_SPACE,
    );

    // Fetch whirepool accounts
    let whirlpool = fetch_whirlpool_account(&rpc_client, whirlpool_pda)
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
    let tick_arrays = fetch_tick_arrays(&rpc_client, tick_pdas)
        .await
        .expect("Tick array accounts not valid");

    // Get quote
    let amount_in = (Decimal::from_f64(input_amount).unwrap_or_default() * decimals_in_pow)
        .to_u64()
        .unwrap_or_default();
    let [amount_a, amount_b] = get_swap_quote(&whirlpool, tick_arrays, amount_in, true, a_to_b);
    let amount_out = Decimal::from(if a_to_b { amount_b } else { amount_a });

    let output_amount = amount_out / decimals_out_pow;
    output_amount.to_f64().unwrap_or_default()
}

pub fn get_swap_quote(
    whirlpool: &Whirlpool,
    tick_arrays: Vec<TickArray>,
    amount: u64,
    amount_specified_is_input: bool,
    a_to_b: bool,
) -> [u64; 2] {
    let ta0_refcell = RefCell::new(tick_arrays[0]);
    let ta1_refcell = RefCell::new(tick_arrays[1]);
    let ta2_refcell = RefCell::new(tick_arrays[2]);
    let mut swap_tick_sequence = SwapTickSequence::new(
        ta0_refcell.borrow_mut(),
        Some(ta1_refcell.borrow_mut()),
        Some(ta2_refcell.borrow_mut()),
    );

    // dummy
    let timestamp = whirlpool.reward_last_updated_timestamp;
    let sqrt_price_limit = if a_to_b {
        MIN_SQRT_PRICE
    } else {
        MAX_SQRT_PRICE
    };

    let swap_update = swap(
        whirlpool,
        &mut swap_tick_sequence,
        amount,
        sqrt_price_limit,
        amount_specified_is_input,
        a_to_b,
        timestamp,
    )
    .unwrap();

    [swap_update.amount_a, swap_update.amount_b]
}

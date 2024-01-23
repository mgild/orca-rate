use std::cell::RefCell;
use whirlpool::manager::swap_manager::swap;
use whirlpool::state::{TickArray, Whirlpool};
use whirlpool::util::SwapTickSequence;

use crate::{MAX_SQRT_PRICE, MIN_SQRT_PRICE};

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

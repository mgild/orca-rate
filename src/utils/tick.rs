use crate::get_tick_array_pda;
use solana_program::pubkey::Pubkey;
use whirlpool::state::{MAX_TICK_INDEX, MIN_TICK_INDEX, TICK_ARRAY_SIZE};

pub fn get_start_tick_index(tick_index: i32, tick_spacing: i32, offset: i32) -> Option<i32> {
    let real_idx =
        f64::floor(f64::from(tick_index) / f64::from(tick_spacing) / f64::from(TICK_ARRAY_SIZE))
            as i32;
    let start_tick_idx = (real_idx + offset) * tick_spacing * TICK_ARRAY_SIZE;

    let ticks_in_array = TICK_ARRAY_SIZE * tick_spacing;
    let min_tick_index = MIN_TICK_INDEX - ((MIN_TICK_INDEX % ticks_in_array) + ticks_in_array);
    if start_tick_idx < min_tick_index || start_tick_idx > MAX_TICK_INDEX {
        None
    } else {
        Some(start_tick_idx)
    }
}

pub fn build_tick_array_pdas(
    tick_current_index: i32,
    tick_spacing: i32,
    a_to_b: bool,
    program_id: Pubkey,
    whirlpool_address: Pubkey,
) -> Vec<Pubkey> {
    let mut pdas: Vec<Pubkey> = vec![];

    let shift: i32 = if a_to_b { 0 } else { tick_spacing };

    let mut offset: i32 = 0;
    for _ in 0..3 {
        match get_start_tick_index(tick_current_index + shift, tick_spacing, offset) {
            Some(start_index) => {
                let pda = get_tick_array_pda(program_id, whirlpool_address, start_index);
                pdas.push(pda);

                if a_to_b {
                    offset = offset - 1;
                } else {
                    offset = offset + 1;
                }
            }
            None => {
                return pdas;
            }
        }
    }

    pdas
}

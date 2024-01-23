use solana_sdk::pubkey::Pubkey;
use std::str::FromStr;

use crate::{PDA_TICK_ARRAY_SEED, PDA_WHIRLPOOL_SEED};

pub fn get_pubkey(address: &str) -> Pubkey {
    Pubkey::from_str(address).unwrap_or_default()
}

pub fn get_whirlpool_pda(
    program_id: Pubkey,
    config_id: Pubkey,
    mint_a: Pubkey,
    mint_b: Pubkey,
    tick_space: u16,
) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[
            PDA_WHIRLPOOL_SEED,
            &config_id.to_bytes(),
            &mint_a.to_bytes(),
            &mint_b.to_bytes(),
            &u16::to_le_bytes(tick_space),
        ],
        &program_id,
    );

    pda
}

pub fn get_tick_array_pda(
    program_id: Pubkey,
    whirlpool_address: Pubkey,
    start_index: i32,
) -> Pubkey {
    let (pda, _) = Pubkey::find_program_address(
        &[
            PDA_TICK_ARRAY_SEED,
            &whirlpool_address.to_bytes(),
            &start_index.to_string().as_bytes(),
        ],
        &program_id,
    );

    pda
}

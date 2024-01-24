use serde;
use serde::{Deserialize, Serialize};
use solana_program::pubkey::Pubkey;
use std::collections::HashMap;
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct Token {
    pub tag: String,
    pub name: String,
    pub mint: Pubkey,
    pub scale: u64,
    pub addr: Pubkey,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct OrcaPool {
    pub address: Pubkey,
    pub nonce: u64,
    pub authority: Pubkey,
    pub pool_token_mint: Pubkey,
    pub pool_token_decimals: u64,
    pub fee_account: Pubkey,
    pub token_ids: Vec<String>,
    pub tokens: HashMap<String, Token>,
    pub fee_structure: JSONFeeStructure,
    pub curve_type: u8,
    #[serde(default)]
    pub amp: u64,
    // to set later
    #[serde(skip)]
    pub pool_amounts: HashMap<String, u128>,
}

#[derive(Deserialize, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct JSONFeeStructure {
    pub trader_fee: Fraction,
    pub owner_fee: Fraction,
}

#[derive(Deserialize, Serialize, Debug, Clone)]

pub struct Fraction {
    pub numerator: u64,
    pub denominator: u64,
}
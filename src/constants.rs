// Mint pubkey & decimials
pub const SOL_MINT: &str = "So11111111111111111111111111111111111111112";
pub const MSOL_MINT: &str = "mSoLzYCxHdYgdzU16g5QSh3i5K3z3KZK7ytfqcJm7So";
pub const USDC_MINT: &str = "EPjFWdd5AufqSSqeM2qN1xzybapC8G4wEGGkZwyTDt1v";
pub const USDT_MINT: &str = "Es9vMFrzaCERmJfrF4H2FYD4KCoNkY11McCe8BenwNYB";

// Whirlpool variables
pub const ORCA_WHIRLPOOL_ID: &str = "whirLbMiicVdio4qvUfM5KAg6Ct8VwpYzGff3uctyCc";
pub const WHIRLPOOL_CONFIG_ID: &str = "2LecshUwdy9xi7meFgHtFJQNSKk4KdTrcpvaB56dP2NQ";
pub const TICK_SPACE: u16 = 64;

pub const PDA_WHIRLPOOL_SEED: &[u8] = b"whirlpool";
pub const PDA_TICK_ARRAY_SEED: &[u8] = b"tick_array";

pub const MIN_SQRT_PRICE: u128 = 4295048016;
pub const MAX_SQRT_PRICE: u128 = 79226673515401279992447579055;

// Legacy pool variables
pub const SOL_USDC_POOL_ADDRESS: &str = "EGZ7tiLeH62TPV1gL8WwbXGzEPa9zmcpVnnkPKKnrE2U";
pub const SOL_USDC_TOKEN_A_DEPOSIT: &str = "ANP74VNsHwSrq9uUSjiSNyNWvf6ZPrKTmE4gHoNd13Lg";
pub const SOL_USDC_TOKEN_B_DEPOSIT: &str = "75HgnSvXbWKZBpZHveX68ZzAhDqMzNDS29X6BGLtxMo1";

pub const ORCA_SWAP_FEE: (u128, u128) = (30, 10000);

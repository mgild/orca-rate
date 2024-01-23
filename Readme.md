# Installation
`cargo build`

# Create .env
 RPC_URL = `mainnet rpc url`

# Modify input & output mint address and decimals, amount

    // You can replace MINT or DECIMALS which defined in constants.rs
    let mint_in = get_pubkey(SOL_MINT);
    let decimals_in = Decimal::from(pow(10, SOL_DECIMALS));
    let mint_out = get_pubkey(BONK_MINT);
    let decimals_out = Decimal::from(pow(10, BONK_DECIMALS));

    // Amount is raw value, not multiple decimals
    let input_amount: f64 = 1.0;

# Run script
`cargo run`

And you can see estimated output amount.
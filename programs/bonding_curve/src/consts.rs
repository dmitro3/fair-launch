pub const INITIAL_PRICE_DIVIDER: u64 = 800_000;       // lamports per one token (without decimal)
pub const INITIAL_LAMPORTS_FOR_POOL: u64 = 10_000_000;   // 0.01SOL
pub const TOKEN_SELL_LIMIT_PERCENT: u64 = 8000;     //  80%
pub const PROPORTION: u64 = 1280;      //  800M token is sold on 500SOL ===> (500 * 2 / 800) = 1.25 ===> 800 : 1.25 = 640 ====> 640 * 2 = 1280


pub const CURVE_CONFIGURATION_SEED: &'static str = "curve_configuration";
pub const BONDING_CURVE_SEED: &'static str = "bonding_curve";
pub const POOL_SEED_PREFIX: &'static str = "liquidity_pool";
pub const SOL_VAULT_PREFIX: &'static str = "liquidity_sol_vault";



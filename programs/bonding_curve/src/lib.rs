use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::*;

declare_id!("3YUv7sv4in2ScU5QEMQpB9mN2ywJEbmNrYUD4ZiW4MS1");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeBondingCurve>,
        creator: Pubkey,
        token: Pubkey,
        fee: f64,
        fee_recipient: Pubkey,
        initial_quorum: u64,
        target_liquidity: u64,
        dao_quorum: u16,
        locked_liquidity: bool,
        governance: Pubkey,
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            creator,
            token,
            fee,
            fee_recipient,
            initial_quorum,
            target_liquidity,
            dao_quorum,
            locked_liquidity,
            governance,
        )
    }

    pub fn create_pool(ctx: Context<CreateLiquidityPool>) -> Result<()> {
        instructions::create_pool(ctx)
    }

    pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
        instructions::buy(ctx, amount)
    }

    pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
        instructions::sell(ctx, amount, bump)
    }
}

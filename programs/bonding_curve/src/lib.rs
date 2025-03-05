use anchor_lang::prelude::*;

pub mod consts;
pub mod errors;
pub mod instructions;
pub mod state;
pub mod utils;

use crate::instructions::*;

declare_id!("2uDeeCCahLYVRzoUrd9NpH1SRgSvY8JB7xiS1Esqktdx");

#[program]
pub mod bonding_curve {
    use super::*;

    pub fn initialize(
        ctx: Context<InitializeBondingCurve>,
        fee: f64,
        fee_recipient: Pubkey,
        initial_quorum: u64,
    ) -> Result<()> {
        instructions::initialize(
            ctx,
            fee,
            fee_recipient,
            initial_quorum
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

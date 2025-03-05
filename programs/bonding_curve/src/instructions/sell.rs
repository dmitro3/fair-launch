use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{CurveConfiguration};
use crate::consts::*;
pub fn sell(ctx: Context<Sell>, amount: u64, bump: u8) -> Result<()> {
    // let pool = &mut ctx.accounts.pool;
    // TODO: Implement sell function
    Ok(())
}

#[derive(Accounts)]
pub struct Sell<'info> {
    #[account(
        mut,
        seeds = [CURVE_CONFIGURATION_SEED.as_bytes(), user.key().as_ref()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,


    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,
}

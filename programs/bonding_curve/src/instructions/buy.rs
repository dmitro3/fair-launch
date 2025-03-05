use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount},
};

use crate::state::{CurveConfiguration, BondingCurve, BondingCurveAccount};
use crate::consts::*;

// todo: base_side 

pub fn buy(ctx: Context<Buy>, amount: u64) -> Result<()> {
    msg!("Trying to buy from the pool");
    // TODO: Implement buy function
    let bonding_curve = &mut ctx.accounts.bonding_curve_account;
    let user = &ctx.accounts.user;
    let system_program = &ctx.accounts.system_program;
    let pool_sol_vault = &mut ctx.accounts.pool_sol_vault;

    let max_pc_amout =  bonding_curve.calculate_buy_cost(amount)?;
    // TODO: update bonding curve account
    bonding_curve.total_supply += amount;
    bonding_curve.reserve_balance += amount;
    bonding_curve.transfer_sol_to_pool(user, pool_sol_vault, amount, system_program)?;

    Ok(())
}

#[derive(Accounts)]
pub struct Buy<'info> {
    #[account(
        mut,
        seeds = [CURVE_CONFIGURATION_SEED.as_bytes()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,

    #[account(
        mut,
        seeds = [BONDING_CURVE_SEED.as_bytes(), token_mint.key().as_ref()],
        bump = bonding_curve_account.bump,
    )]
    pub bonding_curve_account: Box<Account<'info, BondingCurve>>,

    #[account(mut)]
    pub token_mint: Box<Account<'info, Mint>>,

    #[account(
        mut,
        associated_token::mint = token_mint,
        associated_token::authority = bonding_curve_account
    )]
    pub pool_token_account: Box<Account<'info, TokenAccount>>,

    /// CHECK:
    #[account(
        mut,
        seeds = [SOL_VAULT_PREFIX.as_bytes(), token_mint.key().as_ref()],
        bump
    )]
    pub pool_sol_vault: AccountInfo<'info>,

    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user,
    )]
    pub user_token_account: Box<Account<'info, TokenAccount>>,

    #[account(mut)]
    pub user: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
    pub token_program: Program<'info, Token>,
    pub associated_token_program: Program<'info, AssociatedToken>,


}






use crate::{errors::CustomError, state::*};
use anchor_lang::prelude::*;

use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::consts::*;


#[derive(Accounts, Clone)]
pub struct InitializeBondingCurve<'info> {
    #[account(
        init,
        // todo: Calculate correct space for optimizing 
        space = 8+ 3000,
        payer = admin,
        seeds = [CURVE_CONFIGURATION_SEED.as_bytes(), admin.key().as_ref()],
        bump,
    )]
    pub dex_configuration_account: Box<Account<'info, CurveConfiguration>>,


    #[account(
        init,
        // todo : Calculate correct space for optimizing 
        space = 8 +3000,
        payer = admin,
        seeds = [BONDING_CURVE_SEED.as_bytes(), admin.key().as_ref()],
        bump,
    )]
    pub bonding_curve_account: Box<Account<'info, BondingCurve>>,


    #[account(mut)]
    pub admin: Signer<'info>,
    pub rent: Sysvar<'info, Rent>,
    pub system_program: Program<'info, System>,
}




pub fn initialize(
    ctx: Context<InitializeBondingCurve>,
    creator: Pubkey,
    token: Pubkey,
    fees: f64,
    fee_recipient: Pubkey,
    initial_quorum: u64,
    target_liquidity: u64,
    dao_quorum: u16,
    locked_liquidity: bool,
    governance: Pubkey,

) -> Result<()> {
    let dex_config = &mut ctx.accounts.dex_configuration_account;
    let bonding_curve = &mut ctx.accounts.bonding_curve_account;

    if fees < 0_f64 || fees > 100_f64 {
        return err!(CustomError::InvalidFee);
    }

    dex_config.set_inner(CurveConfiguration::new(fees, fee_recipient, initial_quorum));

    bonding_curve.set_inner(BondingCurve::new(creator, token, target_liquidity, dao_quorum, locked_liquidity, governance));

    Ok(())
}
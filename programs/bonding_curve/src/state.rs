
use crate::errors::CustomError;
use anchor_lang::prelude::*;
use anchor_lang::solana_program::native_token::LAMPORTS_PER_SOL;
use anchor_lang::system_program;
use anchor_spl::token::{self, Mint, Token, TokenAccount};
use crate::consts::*;
#[account]
pub struct CurveConfiguration {
    pub fees: f64,
    pub fee_recipient: Pubkey,
    pub initial_quorum: u64,
    pub use_dao: bool,
    pub governance: Pubkey,     // Shared governance contract address
    pub dao_quorum: u16,        // Minimum token quorum (in basis points) for DAO decisions
    pub locked_liquidity: bool, // Whether liquidity is locked
    pub target_liquidity: u64, // Threshold to trigger liquidity addition
    pub fee_percentage: u16,   // Transaction fee in basis points (e.g., 200 = 2%)
    pub fees_enabled: bool,    // Toggle for enabling/disabling fees

}

impl CurveConfiguration {

    // Discriminator (8) + f64 (8)
    pub const ACCOUNT_SIZE: usize = 8 + 32 + 8;

    pub fn new(fees: f64, fee_recipient: Pubkey, initial_quorum: u64, target_liquidity: u64, governance: Pubkey, dao_quorum: u16) -> Self {
        Self {
            fees,
            fee_recipient,
            initial_quorum,
            use_dao: false,
            governance,
            dao_quorum,
            locked_liquidity: false,
            target_liquidity,
            fee_percentage: 0,
            fees_enabled: true,
        }
    }
}

#[account]
pub struct BondingCurve {
    pub creator: Pubkey,
    pub total_supply: u64,     // Tracks the total token supply
    pub reserve_balance: u64,  // Tracks the SOL reserve balance
    pub reserve_token: u64,    // Tracks the token reserve balance
    pub token: Pubkey,         // Public key of the token in the liquidity pool
    pub reserve_ratio: u16,    // Reserve ratio in basis points (default: 50%)
    pub bump: u8,               // Bump seed for PDA
}

impl BondingCurve {
    pub fn new(
        creator: Pubkey,
        token: Pubkey,
        bump: u8,
    ) -> Self {
        Self {
            creator,
            total_supply: 0,
            reserve_balance: 0,
            reserve_token: 0,
            token,
            reserve_ratio: 5000,
            bump,
        }
    }
}


pub trait BondingCurveAccount<'info> {
    fn calculate_buy_cost(&mut self, amount: u64) -> Result<u64>;
    fn calculate_sell_cost(&mut self, amount: u64) -> Result<u64>;

    // Allows adding liquidity by depositing an amount of two tokens and getting back pool shares
    fn add_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    // Allows removing liquidity by burning pool shares and receiving back a proportionate amount of tokens
    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_account: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn buy(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn sell(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()>;


    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()>;

    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()>;


}

impl<'info> BondingCurveAccount<'info> for Account<'info, BondingCurve> {
    fn calculate_buy_cost(&mut self, amount: u64) -> Result<u64> {
        let new_supply = self
            .total_supply
            .checked_add(amount)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let new_supply_squared = (new_supply as u128)
            .checked_mul(new_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let total_supply_squared = (self.total_supply as u128)
            .checked_mul(self.total_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let numerator = new_supply_squared
            .checked_sub(total_supply_squared)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?
            .checked_div(2)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;


        let denominator = (self.reserve_ratio as u128)
            .checked_mul(10000)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;


        let cost = numerator
            .checked_div(denominator)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        if cost > u64::MAX as u128 {
            return Err(CustomError::OverFlowUnderFlowOccured.into());
        }

        Ok(cost as u64)
    }

    fn calculate_sell_cost(&mut self, amount: u64) -> Result<u64> {
        if amount > self.total_supply {
            return Err(CustomError::InsufficientBalance.into());
        }

        let new_supply = self
            .total_supply
            .checked_sub(amount)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let total_supply_squared = (self.total_supply as u128)
            .checked_mul(self.total_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let new_supply_squared = (new_supply as u128)
            .checked_mul(new_supply as u128)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let numerator = total_supply_squared
            .checked_sub(new_supply_squared)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?
            .checked_div(2)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let denominator = (self.reserve_ratio as u128)
            .checked_mul(10000)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        let reward = numerator
            .checked_div(denominator)
            .ok_or(CustomError::OverFlowUnderFlowOccured)?;

        if reward > u64::MAX as u128 {
            return Err(CustomError::OverFlowUnderFlowOccured.into());
        }

        Ok(reward as u64)
    }

    fn buy(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        let amount_out =  self.calculate_buy_cost(amount)?;
        msg!("amount_out {}", amount_out);
    
        // TODO: update bonding curve account
        self.total_supply += amount;
        self.reserve_balance += amount;
        self.transfer_sol_to_pool(authority, pool_sol_vault, amount, system_program)?;
    
        self.transfer_token_from_pool(token_accounts.1, token_accounts.2, amount_out, token_program)?;


        Ok(())
    }

    fn sell(
        &mut self,
        // bonding_configuration_account: &Account<'info, CurveConfiguration>,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        amount: u64,
        bump: u8,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {


        let amount_out = self.calculate_sell_cost(amount)?;
        msg!("reward {}", amount_out);
        if self.reserve_balance < amount_out {
            return err!(CustomError::NotEnoughSolInVault);
        }


        self.total_supply -= amount;
        self.reserve_balance -= amount_out;
        self.transfer_token_to_pool(
            token_accounts.2,
            token_accounts.1,
            amount as u64,
            authority,
            token_program,
        )?;

        self.transfer_sol_from_pool(pool_sol_vault, authority, amount_out, bump, system_program)?;

        Ok(())
    }

    fn add_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        msg!("Adding liquidity to the pool");
        msg!("token_accounts.0.supply {}", token_accounts.0.supply);
        // testing purpose

        self.transfer_token_to_pool(
            token_accounts.2,
            token_accounts.1,
            // token_accounts.0.supply,
            100000000000,
            authority,
            token_program,
        )?;

        self.transfer_sol_to_pool(
            authority,
            pool_sol_vault,
            INITIAL_LAMPORTS_FOR_POOL,
            system_program,
        )?;
        // self.reserve_token += token_accounts.0.supply;
        // testing purpose 
        self.reserve_token += 100000000000;
        self.reserve_balance += INITIAL_LAMPORTS_FOR_POOL;

        Ok(())
    }

    // Allows removing liquidity by burning pool shares and receiving back a proportionate amount of tokens
    fn remove_liquidity(
        &mut self,
        token_accounts: (
            &mut Account<'info, Mint>,
            &mut Account<'info, TokenAccount>,
            &mut Account<'info, TokenAccount>,
        ),
        pool_sol_vault: &mut AccountInfo<'info>,
        authority: &Signer<'info>,
        bump: u8,
        token_program: &Program<'info, Token>,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        self.transfer_token_from_pool(
            token_accounts.1,
            token_accounts.2,
            token_accounts.1.amount as u64,
            token_program,
        )?;
        let amount = pool_sol_vault.to_account_info().lamports() as u64;
        self.transfer_sol_from_pool(pool_sol_vault, authority, amount, bump, system_program)?;
        Ok(())
    }


    fn transfer_sol_to_pool(
        &self,
        from: &Signer<'info>,
        to: &mut AccountInfo<'info>,
        amount: u64,
        system_program: &Program<'info, System>,
    ) -> Result<()> {

        system_program::transfer(
            CpiContext::new(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                },
            ),
            amount,
        )?;

        Ok(())
    }

    fn transfer_sol_from_pool(
        &self,
        from: &mut AccountInfo<'info>,
        to: &Signer<'info>,
        amount: u64,
        bump: u8,
        system_program: &Program<'info, System>,
    ) -> Result<()> {
        system_program::transfer(
            CpiContext::new_with_signer(
                system_program.to_account_info(),
                system_program::Transfer {
                    from: from.clone(),
                    to: to.to_account_info().clone(),
                },
                &[&[
                    SOL_VAULT_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    &[bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_token_from_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new_with_signer(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: self.to_account_info(),
                },
                &[&[
                    POOL_SEED_PREFIX.as_bytes(),
                    self.token.key().as_ref(),
                    &[self.bump],
                ]],
            ),
            amount,
        )?;
        Ok(())
    }

    fn transfer_token_to_pool(
        &self,
        from: &Account<'info, TokenAccount>,
        to: &Account<'info, TokenAccount>,
        amount: u64,
        authority: &Signer<'info>,
        token_program: &Program<'info, Token>,
    ) -> Result<()> {
        token::transfer(
            CpiContext::new(
                token_program.to_account_info(),
                token::Transfer {
                    from: from.to_account_info(),
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )?;
        Ok(())
    }

}

pub trait CurveConfigurationAccount<'info> {
    fn toggle_dao(&mut self) -> Result<()>;
    fn update_fee(&mut self, fee: f64) -> Result<()>;
    fn update_fee_recipient(&mut self, fee_recipient: Pubkey) -> Result<()>;
}

impl<'info> CurveConfigurationAccount<'info> for Account<'info, CurveConfiguration> {
    fn toggle_dao(&mut self) -> Result<()> {
        if self.use_dao {
            return err!(CustomError::DAOAlreadyActivated);
        }
        self.use_dao = true;
        Ok(())
    }

    fn update_fee(&mut self, new_fee: f64) -> Result<()> {
        // Maximum fee is 10%
        if new_fee <= 1000_f64 {
            return err!(CustomError::InvalidFee);
        }
        self.fees = new_fee;
        Ok(())
    }

    fn update_fee_recipient(&mut self, new_fee_recipient: Pubkey) -> Result<()> {
        self.fee_recipient = new_fee_recipient;
        Ok(())
    }
}

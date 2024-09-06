use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod helper;
pub mod instructions;

use instructions::*;

pub mod states;
use states::*;

declare_id!("6EaqAvkpifYeNsQ1SqPiUQajFd7pBBwqgstPLhwkBC3g");

#[program]
pub mod forecast_exchange {
    use super::*;

    pub fn initialize(
        ctx: Context<Initialize>,
        point_mint: Pubkey,
        token_mint: Pubkey,
    ) -> Result<()> {
        instructions::initialize(ctx, point_mint, token_mint)?;
        Ok(())
    }

    pub fn buy_point(ctx: Context<BuyPoint>, amount: u64) -> Result<()> {
        instructions::buy_point(ctx, amount)?;
        Ok(())
    }

    pub fn sell_point(ctx: Context<SellPoint>, amount: u64) -> Result<()> {
        instructions::sell_point(ctx, amount)?;
        Ok(())
    }
}

use anchor_lang::prelude::*;

pub mod constant;
pub mod error;
pub mod helper;
pub mod instructions;

use instructions::*;

pub mod states;
use states::*;

pub mod message;
use message::*;

declare_id!("FVWe7wMvbWvnk1Vo4viZLejdK7oFiy5S5csLAXazbuQG");

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

    pub fn initialize_bridge(ctx: Context<InitializeBridge>, relayer_fee: u32, relayer_fee_precision: u32) -> Result<()> {
        instructions::initialize_bridge(ctx, relayer_fee, relayer_fee_precision)?;
        Ok(())
    }

    pub fn transfer_cross_chain(ctx: Context<TransferCrossChain>, batch_id: u32, amount: u64, recipient_address: [u8; 32], recipient_chain: u16, recipent_contract: [u8; 32]) -> Result<()> {
        instructions::transfer_cross_chain(ctx, batch_id, amount, recipient_address, recipient_chain, recipent_contract)?;
        Ok(())
    }
}

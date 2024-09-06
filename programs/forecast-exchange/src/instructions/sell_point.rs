use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
 token_interface::{Mint, TokenAccount, TokenInterface},
};

use crate::{
    error::ProgramErrorCode, helper::{transfer_token_or_point_from_owner_to_user, transfer_token_or_point_to_pool},
    ConfigAccount, CONFIG_SEED,
};

#[derive(Accounts)]
pub struct SellPoint<'info> {
    #[account(mut)]
    pub user: Signer<'info>,
    #[account(
        mut,
        constraint = config_account.token_mint == token_mint.key(),
        constraint = config_account.point_mint == point_mint.key()
    )]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub token_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub point_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub user_point_account : Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(        
        init_if_needed,
        payer = user,
        associated_token::mint = token_mint,
        associated_token::authority = user
    )]
    pub user_token_account: InterfaceAccount<'info, TokenAccount>,
    #[account(mut)]
    pub owner_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = user,
        associated_token::mint = point_mint,
        associated_token::authority = config_account
    )]
    pub vault_point_account: InterfaceAccount<'info, TokenAccount>,

    pub token_program: Interface<'info, TokenInterface>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct Sold {
    pub user: Pubkey,
    pub amount: u64,
}

pub fn sell_point(ctx: Context<SellPoint>, amount: u64) -> Result<()> {

    require!(amount > 0, ProgramErrorCode::AmountZero);
    
   // Transfer point from user to program pool
    transfer_token_or_point_to_pool(
        ctx.accounts.user_point_account.to_account_info(),
        ctx.accounts.vault_point_account.to_account_info(),
        ctx.accounts.user.to_account_info(),
        ctx.accounts.token_program.to_account_info(),
        amount,
    )?;

    let seeds = &CONFIG_SEED.as_bytes();
    let bump = ctx.accounts.config_account.bump;
    let signer: &[&[&[u8]]] = &[&[seeds, &[bump]]];

    //Transfer token from owner to user
    transfer_token_or_point_from_owner_to_user(
        ctx.accounts.owner_token_account.to_account_info(), 
        ctx.accounts.user_token_account.to_account_info(), 
        ctx.accounts.config_account.to_account_info(), 
        ctx.accounts.token_program.to_account_info(), 
        signer, 
        amount
    )?;

    emit!(Sold {
        user: ctx.accounts.user.key(),
        amount,
    });

    Ok(())
}

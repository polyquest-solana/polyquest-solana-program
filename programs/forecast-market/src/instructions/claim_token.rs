use std::ops::DerefMut;

use anchor_lang::prelude::*;
use anchor_spl::token_interface::{Mint, Token2022, TokenAccount};
use anchor_spl::{associated_token::AssociatedToken, token::Token};

use crate::helper::{get_transfer_inverse_fee, transfer_from_pool_vault_to_user};
use crate::{
    constant::MAX_PERCENTAGE_BASIS_POINTS, error::ProgramErrorCode,
    helper::calculate_reward_amount, AnswerAccount, BettingAccount, ConfigAccount, MarketAccount,
    MarketStatus, CONFIG_SEED, MARKET_SEED,
};

#[derive(Accounts)]
pub struct ClaimToken<'info> {
    #[account(mut)]
    pub voter: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub bet_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub reward_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(mut)]
    pub user_bet_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = bet_mint,
        token::authority = market_account
    )]
    pub vault_bet_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        init_if_needed,
        payer = voter,
        associated_token::mint = reward_mint,
        associated_token::authority = voter
    )]
    pub user_reward_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
        mut,
        token::mint = reward_mint,
        token::authority = config_account
    )]
    pub vault_reward_token_account: Box<InterfaceAccount<'info, TokenAccount>>,
    #[account(
      mut,
      constraint = market_account.status == MarketStatus::Success || market_account.status == MarketStatus::Adjourn @ ProgramErrorCode::CannotClaimToken,
      constraint = market_account.bet_mint == bet_mint.key() @ ProgramErrorCode::InvalidBetMint,
    )]
    pub market_account: Box<Account<'info, MarketAccount>>,
    #[account(
        mut,
        close = voter
    )]
    pub bet_account: Box<Account<'info, BettingAccount>>,
    #[account(mut)]
    pub answer_account: Box<Account<'info, AnswerAccount>>,

    pub token_program: Program<'info, Token>,

    pub token_2022_program: Program<'info, Token2022>,

    pub associated_token_program: Program<'info, AssociatedToken>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct TokenClaimed {
    pub receiver: Pubkey,
    pub market_key: u64,
    pub betting_key: u64,
    pub received_tokens: u64,
}

#[event]
pub struct RewardClaimed {
    pub receiver: Pubkey,
    pub amount: u64,
}

pub fn claim_token(ctx: Context<ClaimToken>) -> Result<()> {
    let market_account = ctx.accounts.market_account.deref_mut();

    let config_account = &ctx.accounts.config_account;
    let betting_account = &mut ctx.accounts.bet_account;
    let answer_account = &ctx.accounts.answer_account;

    let correct_answer_key = market_account.correct_answer_key;
    let betting_tokens = betting_account.tokens as u128;
    let answer_key = betting_account.answer_key;
    let finish_time = market_account.finish_time;

    let mut percentage = 0;

    if market_account.status == MarketStatus::Success
        && betting_account.answer_key == correct_answer_key
    {
        let mut correct_answer_total_tokens: u128 = 0;
        for answer in &answer_account.answers {
            if answer.answer_key == correct_answer_key {
                correct_answer_total_tokens = answer.answer_total_tokens as u128;
                break;
            }
        }

        let market_reward_base_tokens = market_account.market_reward_base_tokens as u128;
        percentage = market_reward_base_tokens
            .checked_mul(MAX_PERCENTAGE_BASIS_POINTS)
            .and_then(|result| result.checked_div(correct_answer_total_tokens))
            .ok_or(ProgramErrorCode::MathOperationError)?;
    } else if market_account.status == MarketStatus::Adjourn {
        percentage = MAX_PERCENTAGE_BASIS_POINTS;
        let answer_exists = answer_account
            .answers
            .iter()
            .any(|answer| answer.answer_key == answer_key);

        if !answer_exists {
            return Err(ProgramErrorCode::InvalidAnswerKey.into());
        }
    }

    let receive_tokens = betting_tokens
        .checked_mul(percentage)
        .and_then(|result| result.checked_div(MAX_PERCENTAGE_BASIS_POINTS))
        .ok_or(ProgramErrorCode::MathOperationError)?;

    //dividend token to user
    market_account.market_remain_tokens =
        market_account.market_remain_tokens - receive_tokens as u64;

    if receive_tokens > 0 {
        let bet_seeds: &[&[u8]] = &[
            MARKET_SEED.as_bytes(),
            &ctx.accounts.market_account.market_key.to_le_bytes(),
            &[ctx.accounts.market_account.bump],
        ];

        let amount_transfer_fee =
            get_transfer_inverse_fee(ctx.accounts.bet_mint.clone(), receive_tokens as u64).unwrap();

        transfer_from_pool_vault_to_user(
            &ctx.accounts.vault_bet_token_account.to_account_info(),
            &ctx.accounts.user_bet_token_account.to_account_info(),
            ctx.accounts.bet_mint.clone(),
            &ctx.accounts.market_account.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            Some(&ctx.accounts.token_2022_program.to_account_info()),
            amount_transfer_fee as u64,
            &[&bet_seeds],
        )?;

        betting_account.tokens = 0;
        emit!(TokenClaimed {
            receiver: ctx.accounts.voter.key(),
            market_key: ctx.accounts.market_account.market_key,
            betting_key: betting_account.answer_key,
            received_tokens: receive_tokens as u64,
        });
    }

    let reward_seeds: &[&[u8]] = &[CONFIG_SEED.as_bytes(), &[ctx.accounts.config_account.bump]];

    let reward_amount = calculate_reward_amount(
        betting_account.tokens,
        config_account.reward_apr,
        betting_account.create_time,
        finish_time,
    )?;

    if reward_amount > 0 {
        transfer_from_pool_vault_to_user(
            &ctx.accounts.vault_reward_token_account.to_account_info(),
            &ctx.accounts.user_reward_token_account.to_account_info(),
            ctx.accounts.reward_mint.clone(),
            &ctx.accounts.config_account.to_account_info(),
            &ctx.accounts.token_program.to_account_info(),
            Some(&ctx.accounts.token_2022_program.to_account_info()),
            reward_amount,
            &[&reward_seeds],
        )?;

        emit!(RewardClaimed {
            receiver: ctx.accounts.voter.key(),
            amount: reward_amount
        })
    }

    Ok(())
}

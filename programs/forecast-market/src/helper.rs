use crate::{
    constant::{
        ADJOURN_MARKET_VALIDITY_DATE, BASIS_POINTS, SECONDS_IN_A_YEAR, SUCCESS_MARKET_VALIDITY_DATE,
    },
    error::ProgramErrorCode,
    MarketAccount, MarketStatus,
};
use anchor_lang::prelude::*;
use anchor_spl::{
    token::{self, Token},
    token_2022::{
        self,
        spl_token_2022::{
            self,
            extension::{
                transfer_fee::{TransferFeeConfig, MAX_FEE_BASIS_POINTS},
                BaseStateWithExtensions, ExtensionType, StateWithExtensions,
            },
        },
    },
    token_interface::{Mint, TokenAccount},
};

pub fn transfer_from_user_to_pool_vault<'info>(
    from: &AccountInfo<'info>,
    to_vault: &AccountInfo<'info>,
    mint: Box<InterfaceAccount<'info, Mint>>,
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    token_program_2022: Option<&AccountInfo<'info>>,
    amount: u64,
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let mut token_program_info = token_program.to_account_info();
    let from_token_info = from.to_account_info();

    if let Some(token_program_2022) = token_program_2022 {
        if from_token_info.owner == token_program_2022.key {
            token_program_info = token_program_2022.to_account_info();
        }
        token_2022::transfer_checked(
            CpiContext::new(
                token_program_info,
                token_2022::TransferChecked {
                    from: from_token_info,
                    to: to_vault.to_account_info(),
                    authority: authority.to_account_info(),
                    mint: mint.to_account_info(),
                },
            ),
            amount,
            mint.decimals, 
        )
    } else {
        token::transfer(
            CpiContext::new(
                token_program_info,
                token::Transfer {
                    from: from_token_info,
                    to: to_vault.to_account_info(),
                    authority: authority.to_account_info(),
                },
            ),
            amount,
        )
    }
}


pub fn transfer_from_pool_vault_to_user<'info>(
    from_vault: &AccountInfo<'info>,
    to: &AccountInfo<'info>,
    mint: Box<InterfaceAccount<'info, Mint>>, 
    authority: &AccountInfo<'info>,
    token_program: &AccountInfo<'info>,
    token_program_2022: Option<&AccountInfo<'info>>,
    amount: u64,
    signer_seeds: &[&[&[u8]]],
) -> Result<()> {
    if amount == 0 {
        return Ok(());
    }

    let mut token_program_info = token_program.to_account_info();
    let from_vault_info = from_vault.to_account_info();

    if let Some(token_program_2022) = token_program_2022 {
        if from_vault_info.owner == token_program_2022.key {
            token_program_info = token_program_2022.to_account_info();
        }

        token_2022::transfer_checked(
            CpiContext::new_with_signer(
                token_program_info,
                token_2022::TransferChecked {
                    from: from_vault_info,
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                    mint: mint.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
            mint.decimals,
        )
    } else {
        token::transfer(
            CpiContext::new_with_signer(
                token_program_info,
                token::Transfer {
                    from: from_vault_info,
                    to: to.to_account_info(),
                    authority: authority.to_account_info(),
                },
                signer_seeds,
            ),
            amount,
        )
    }
}


pub fn close_spl_account<'a, 'b, 'c, 'info>(
    owner: &AccountInfo<'info>,
    destination: &AccountInfo<'info>,
    close_account: &InterfaceAccount<'info, TokenAccount>,
    token_program: &Program<'info, Token>,
    // token_program_2022: &Program<'info, Token2022>,
    signers_seeds: &[&[&[u8]]],
) -> Result<()> {
    let token_program_info = token_program.to_account_info();
    let close_account_info = close_account.to_account_info();
    // if close_account_info.owner == token_program_2022.key {
    //     token_program_info = token_program_2022.to_account_info()
    // }

    token_2022::close_account(CpiContext::new_with_signer(
        token_program_info,
        token_2022::CloseAccount {
            account: close_account_info,
            destination: destination.to_account_info(),
            authority: owner.to_account_info(),
        },
        signers_seeds,
    ))
}

/// Calculate the fee for input amount
pub fn get_transfer_fee(
    mint_account: Box<InterfaceAccount<Mint>>,
    pre_fee_amount: u64,
) -> Result<u64> {
    let mint_info = mint_account.to_account_info();
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        transfer_fee_config
            .calculate_epoch_fee(Clock::get()?.epoch, pre_fee_amount)
            .unwrap()
    } else {
        0
    };
    Ok(fee)
}

/// Calculate the fee for output amount
pub fn get_transfer_inverse_fee(
    mint_account: Box<InterfaceAccount<Mint>>,
    post_fee_amount: u64,
) -> Result<u64> {
    let mint_info: AccountInfo<'_> = mint_account.to_account_info();
    if *mint_info.owner == Token::id() {
        return Ok(0);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;

    let fee = if let Ok(transfer_fee_config) = mint.get_extension::<TransferFeeConfig>() {
        let epoch = Clock::get()?.epoch;

        let transfer_fee = transfer_fee_config.get_epoch_fee(epoch);
        if u16::from(transfer_fee.transfer_fee_basis_points) == MAX_FEE_BASIS_POINTS {
            u64::from(transfer_fee.maximum_fee)
        } else {
            transfer_fee_config
                .calculate_inverse_epoch_fee(epoch, post_fee_amount)
                .unwrap()
        }
    } else {
        0
    };
    Ok(fee)
}

pub fn calculate_reward_amount(
    token_amount: u64,
    reward_apr: u64,
    create_time: u64,
    finish_time: u64,
) -> Result<u64> {
    require!(
        finish_time >= create_time,
        ProgramErrorCode::InvalidTimeRange
    );

    let time_staked = finish_time - create_time;

    msg!("time_staked: {}", time_staked);

    let token_amount_128 = token_amount as u128;
    let reward_apr_128 = reward_apr as u128;
    let basis_points_128 = BASIS_POINTS as u128;
    let time_staked_128 = time_staked as u128;
    let seconds_in_a_year_128 = SECONDS_IN_A_YEAR as u128;

    let reward_amount_128 = token_amount_128
        .checked_mul(reward_apr_128)
        .and_then(|r| r.checked_div(basis_points_128))
        .and_then(|r| r.checked_mul(time_staked_128))
        .and_then(|r| r.checked_div(seconds_in_a_year_128))
        .ok_or(ProgramErrorCode::Overflow)?;

    let reward_amount = reward_amount_128 as u64;

    Ok(reward_amount)
}

pub fn is_retrieve_available(market_account: &MarketAccount, clock: &Clock) -> Result<bool> {
    require!(
        market_account.status == MarketStatus::Success
            || market_account.status == MarketStatus::Adjourn,
        ProgramErrorCode::CannotRetrieveToken
    );

    let diff = if market_account.status == MarketStatus::Success {
        clock.unix_timestamp as u64 - market_account.success_time
    } else {
        clock.unix_timestamp as u64 - market_account.adjourn_time
    };

    let is_available = if market_account.status == MarketStatus::Success {
        diff > SUCCESS_MARKET_VALIDITY_DATE
    } else {
        diff > ADJOURN_MARKET_VALIDITY_DATE
    };

    Ok(is_available)
}

pub fn is_supported_mint(mint_account: &InterfaceAccount<Mint>) -> Result<bool> {
    let mint_info = mint_account.to_account_info();
    if *mint_info.owner == Token::id() {
        return Ok(true);
    }
    let mint_data = mint_info.try_borrow_data()?;
    let mint = StateWithExtensions::<spl_token_2022::state::Mint>::unpack(&mint_data)?;
    let extensions = mint.get_extension_types()?;
    for e in extensions {
        if e != ExtensionType::TransferFeeConfig
            && e != ExtensionType::MetadataPointer
            && e != ExtensionType::TokenMetadata
        {
            return Ok(false);
        }
    }
    Ok(true)
}

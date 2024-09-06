use anchor_lang::prelude::*;
use anchor_spl::token;

pub fn transfer_token_or_point_to_pool<'info>(
    from_pool: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    amount: u64,
) -> Result<()> {
    let token_program_info = token_program.to_account_info();
    let from_pool_info: AccountInfo = from_pool.to_account_info();

    token::transfer(
        CpiContext::new(
            token_program_info,
            token::Transfer {
                from: from_pool_info,
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
        ),
        amount,
    )
}


pub fn transfer_token_or_point_from_owner_to_user<'info>(
    from_owner: AccountInfo<'info>,
    to: AccountInfo<'info>,
    authority: AccountInfo<'info>,
    token_program: AccountInfo<'info>,
    signer_seeds: &[&[&[u8]]],
    amount: u64,
) -> Result<()> {
    let token_program_info = token_program.to_account_info();
    let from_owner_info: AccountInfo = from_owner.to_account_info();

    token::transfer(
        CpiContext::new_with_signer(
            token_program_info,
            token::Transfer {
                from: from_owner_info,
                to: to.to_account_info(),
                authority: authority.to_account_info(),
            },
            signer_seeds,
        ),
        amount,
    )
}
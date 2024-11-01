use std::ops::DerefMut;

use anchor_lang::prelude::*;
// use anchor_spl::{associated_token::AssociatedToken, token::Token};

use anchor_spl::token_interface::Mint;
use wormhole_anchor_sdk::wormhole::{self, program::Wormhole};

// use crate::helper::{get_transfer_inverse_fee, transfer_from_user_to_pool_vault};
use crate::{
    error::ProgramErrorCode, message::{PolyQuestMessage, PolyQuestVaa, MESSAGE_MAX_LENGTH}, AnswerAccount, BettingCrossChainAccount, BettingCrossChainData, ConfigAccount, ForeignEmitter, MarketAccount, MarketStatus, Received, BETTING_CROSS_CHAIN_SEED
};
#[derive(Accounts)]
#[instruction(answer_key: u64, vaa_hash: [u8; 32])]
pub struct BetCrossChain<'info> {
    #[account(mut)]
    pub polyquest_owner: Signer<'info>,
    #[account(mut)]
    pub config_account: Account<'info, ConfigAccount>,
    #[account(mut)]
    pub bet_mint: Box<InterfaceAccount<'info, Mint>>,
    #[account(
      mut,
      constraint = market_account.status == MarketStatus::Approve @ ProgramErrorCode::MarketNotApproved
    )]
    pub market_account: Box<Account<'info, MarketAccount>>,
    #[account(
      mut,
      constraint = market_account.exist == true @ ProgramErrorCode::AnswerNotExists,
    )]
    pub answer_account: Box<Account<'info, AnswerAccount>>,
    pub wormhole_program: Program<'info, Wormhole>,

    #[account(
        seeds = [
            wormhole::SEED_PREFIX_POSTED_VAA,
            &vaa_hash
        ],
        bump,
        seeds::program = wormhole_program.key
    )]
    /// Verified Wormhole message account. The Wormhole program verified
    /// signatures and posted the account data here. Read-only.
    pub posted: Account<'info, PolyQuestVaa>,
    #[account(
      init_if_needed,
      payer = polyquest_owner,
      space = BettingCrossChainAccount::MAX_SIZE,
      seeds = [BETTING_CROSS_CHAIN_SEED.as_bytes(), &posted.emitter_chain().to_le_bytes()[..],
           &posted.sequence().to_le_bytes()[..]],
      bump,
    )]
    pub bet_cross_chain_account: Box<Account<'info, BettingCrossChainAccount>>,
    // #[account(
    //     seeds = [
    //         ForeignEmitter::SEED_PREFIX,
    //         &posted.emitter_chain().to_le_bytes()[..]
    //     ],
    //     bump,
    //     constraint = foreign_emitter.verify(posted.emitter_address()) @ ProgramErrorCode::InvalidForeignEmitter
    // )]
    // /// Foreign emitter account. The posted message's `emitter_address` must
    // /// agree with the one we have registered for this message's `emitter_chain`
    // /// (chain ID). Read-only.
    // pub foreign_emitter: Account<'info, ForeignEmitter>,
    #[account(
        init_if_needed,
        payer = polyquest_owner,
        seeds = [
            Received::SEED_PREFIX,
            &posted.emitter_chain().to_le_bytes()[..],
           &posted.sequence().to_le_bytes()[..]],
        bump,
        space = Received::MAXIMUM_SIZE
    )]
    pub received: Box<Account<'info, Received>>,

    pub system_program: Program<'info, System>,
}

#[event]
pub struct BetCrossChainPlaced {
    pub voter_wallet_address: String,
    pub chain_id: u16,
    pub market_key: u64,
    pub answer_key: u64,
}

pub fn bet_cross_chain(ctx: Context<BetCrossChain>, answer_key: u64, vaa_hash: [u8; 32]) -> Result<()> {
    let posted_message = &ctx.accounts.posted;
    if let PolyQuestMessage::Message { message } = posted_message.data() {
        require!(
            message.len() <= MESSAGE_MAX_LENGTH,
            ProgramErrorCode::InvalidMessage,
        );
        let json = String::from_utf8(message.to_owned()).map_err(|_| ProgramError::InvalidInstructionData)?;

        // Parse `JSON` thành `BettingCrossChainData`, trả về lỗi nếu không khớp
        let data = BettingCrossChainData::from_json(&json)
        .map_err(|_| ProgramErrorCode::InvalidMessage)?;

        let received = &mut ctx.accounts.received;
        received.batch_id = posted_message.batch_id();
        received.wormhole_message_hash = vaa_hash;

        let betting_cross_chain_account = ctx.accounts.bet_cross_chain_account.deref_mut();
        let answer_account = ctx.accounts.answer_account.deref_mut();    
        if !answer_account
            .answers
            .iter()
            .any(|answer| answer.answer_key == data.answer_key)
        {
            return Err(ProgramErrorCode::AnswerNotExists.into());
        }

        // Update the specific answer's total tokens
        for answer in answer_account.answers.iter_mut() {
            if answer.answer_key == data.answer_key {
                // answer.answer_total_tokens += amount;
                break;
            }
        }

        betting_cross_chain_account.bump = ctx.bumps.bet_cross_chain_account;
        betting_cross_chain_account.market_key = data.market_key;
        betting_cross_chain_account.answer_key = data.answer_key;
        betting_cross_chain_account.chain_id = data.chain_id;
        betting_cross_chain_account.voter_wallet_address = data.voter_wallet_address;
        betting_cross_chain_account.token_address = data.token_address;
        betting_cross_chain_account.tokens += data.tokens;
        betting_cross_chain_account.create_time = data.create_time;
        betting_cross_chain_account.exist = true;


        emit!(BetCrossChainPlaced {
            voter_wallet_address: format!("{:?}", data.voter_wallet_address), // hoặc xử lý theo cách mong muốn
            chain_id: data.chain_id,
            market_key: data.market_key,
            answer_key: data.answer_key,
        });
        Ok(())
    } else {
        Err(ProgramErrorCode::InvalidMessage.into())
    }
}

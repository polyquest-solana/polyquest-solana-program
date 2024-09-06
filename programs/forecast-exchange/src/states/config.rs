use anchor_lang::prelude::*;

pub const CONFIG_SEED: &str = "config";
#[account]
pub struct ConfigAccount {
    pub bump: u8,
    pub is_initialized: bool,
    pub owner: Pubkey,
    pub point_mint: Pubkey,
    pub token_mint: Pubkey,
}

impl ConfigAccount {
    pub const LEN: usize = 8 // Account discriminator added by Anchor for each account
            + 1 // bump
            + 1 //is_initialized
            + 32 //owner
            + 32 //point_mint
            + 32; //token_mint
}

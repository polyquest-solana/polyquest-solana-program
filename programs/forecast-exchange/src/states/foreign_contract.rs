use anchor_lang::prelude::*;

use crate::PostedPolyQuestTokenMessage;

#[account]
#[derive(Default)]
/// Foreign emitter account data.
pub struct ForeignContract {
    /// Emitter chain. Cannot equal `1` (Solana's Chain ID).
    pub chain: u16,
    /// Emitter address. Cannot be zero address.
    pub address: [u8; 32],
}

impl ForeignContract {
    pub const MAXIMUM_SIZE: usize = 8 // discriminator
        + 2 // chain
        + 32 // address
    ;
    /// AKA `b"foreign_contract"`.
    pub const SEED_PREFIX: &'static [u8; 16] = b"foreign_contract";

    /// Convenience method to check whether an address equals the one saved in
    /// this account.
    pub fn verify(&self, vaa: &PostedPolyQuestTokenMessage) -> bool {
        vaa.emitter_chain() == self.chain && *vaa.data().from_address() == self.address
    }
}

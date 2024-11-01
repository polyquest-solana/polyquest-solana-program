use anchor_lang::prelude::*;
use serde_json::Value;
use std::str::FromStr;

pub const BETTING_CROSS_CHAIN_SEED: &str = "betting_cross_chain";

#[account]
#[derive(Debug)]

pub struct BettingCrossChainAccount {
    pub bump: u8, //bump for identify
    pub market_key: u64,
    pub answer_key: u64,
    pub create_time: u64,
    pub exist: bool,
    pub chain_id: u16,
    pub voter_wallet_address: [u8; 32],
    pub token_address: [u8; 32],
    pub tokens: u64,
}

impl BettingCrossChainAccount {
    pub const MAX_SIZE: usize = 8 + // discriminator
        1 + // bump
        8 + // market_key
        8 + // answer_key
        8 + // create_time (i64)
        1 + // exist (bool)
        2 + // chainid (Chain)
        32 + // voter_wallet_address
        32 + // token_address
        8; // tokens
}
#[derive(Debug)]
pub struct BettingCrossChainData {
    pub market_key: u64,
    pub answer_key: u64,
    pub create_time: u64,
    pub chain_id: u16,
    pub voter_wallet_address: [u8; 32],
    pub token_address: [u8; 32],
    pub tokens: u64,
}

impl BettingCrossChainData {
    pub fn from_json(json_str: &str) -> std::result::Result<Self, ProgramError> {
        let json_value: Value = serde_json::from_str(json_str)
            .map_err(|_| ProgramError::InvalidInstructionData)?;
        println!("{}", json_value);
        let market_key = u64::from_str_radix(
            json_value["marketKey"]
                .as_str()
                .ok_or(ProgramError::InvalidInstructionData)?,
            16,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;
        println!("Parsed market_key: {:?}", market_key);


        let answer_key = u64::from_str_radix(
            json_value["answerKey"]
                .as_str()
                .ok_or(ProgramError::InvalidInstructionData)?,
            16,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        print!("answer_key: {:?}", answer_key);

        let create_time = u64::from_str_radix(
            json_value["createTime"]
                .as_str()
                .ok_or(ProgramError::InvalidInstructionData)?,
            16,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        let chain_id = json_value["chainId"]
            .as_u64()
            .ok_or(ProgramError::InvalidInstructionData)? as u16;

        let voter_wallet_address = json_value["voterWalletAddress"]
            .as_array()
            .ok_or(ProgramError::InvalidInstructionData)?
            .iter()
            .map(|x| x.as_u64().unwrap_or(0) as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let token_address = json_value["tokenAddress"]
            .as_array()
            .ok_or(ProgramError::InvalidInstructionData)?
            .iter()
            .map(|x| x.as_u64().unwrap_or(0) as u8)
            .collect::<Vec<u8>>()
            .try_into()
            .map_err(|_| ProgramError::InvalidInstructionData)?;

        let tokens = u64::from_str_radix(
            json_value["tokens"]
                .as_str()
                .ok_or(ProgramError::InvalidInstructionData)?,
            16,
        )
        .map_err(|_| ProgramError::InvalidInstructionData)?;

        Ok(Self {
            market_key,
            answer_key,
            create_time,
            chain_id,
            voter_wallet_address,
            token_address,
            tokens,
        })
    }
}

#[account]
#[derive(Default)]
/// Received account.
pub struct Received {
    /// AKA nonce. Should always be zero in this example, but we save it anyway.
    pub batch_id: u32,
    /// Keccak256 hash of verified Wormhole message.
    pub wormhole_message_hash: [u8; 32],
}

impl Received {
    pub const MAXIMUM_SIZE: usize = 8 // discriminator
        + 4 // batch_id
        + 32 // wormhole_message_hash
    ;
    /// AKA `b"received"`.
    pub const SEED_PREFIX: &'static [u8; 8] = b"received";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_from_json() {
        let json_str = r#"{"marketKey":"7a","answerKey":"02","createTime":"6155df00","chainId":43114,"voterWalletAddress":[172,187,215,151,152,118,139,151,46,167,4,198,27,169,1,60,109,71,208,189,89,147,54,195,161,73,91,137,70,140,176,74],"tokenAddress":[27,110,67,169,207,203,160,199,123,93,58,192,90,9,215,127,108,29,18,120,203,229,67,175,169,124,218,187,222,190,53,189],"tokens":"03e8"}"#;

        let data = BettingCrossChainData::from_json(json_str).expect("Failed to parse JSON");

        println!("market_key: {:?}", data.market_key);
        println!("answer_key: {:?}", data.answer_key);
        println!("create_time: {:?}", data.create_time);
        println!("chain_id: {:?}", data.chain_id);
        println!("voter_wallet_address: {:?}", data.voter_wallet_address);
        println!("token_address: {:?}", data.token_address);
        println!("tokens: {:?}", data.tokens);
        // assert_eq!(data.market_key, 79);
        assert_eq!(data.answer_key, 2);
        assert_eq!(data.create_time, 0x6155df00); // 1633017600 in hex
        assert_eq!(data.chain_id, 43114);

        assert_eq!(
            data.voter_wallet_address,
            [
                172, 187, 215, 151, 152, 118, 139, 151, 46, 167, 4, 198, 27, 169, 1, 60, 109, 71,
                208, 189, 89, 147, 54, 195, 161, 73, 91, 137, 70, 140, 176, 74
            ]
        );

        assert_eq!(
            data.token_address,
            [
                27, 110, 67, 169, 207, 203, 160, 199, 123, 93, 58, 192, 90, 9, 215, 127, 108, 29,
                18, 120, 203, 229, 67, 175, 169, 124, 218, 187, 222, 190, 53, 189
            ]
        );

        assert_eq!(data.tokens, 1000); // 0x03e8 in hex
    }
}
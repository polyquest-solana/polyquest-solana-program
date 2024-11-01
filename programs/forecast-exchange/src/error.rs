use anchor_lang::prelude::*;

#[error_code]
pub enum ProgramErrorCode {
    #[msg("The configuration account is already initialized.")]
    AlreadyInitialized,
    #[msg("You are not authorized to perform this action.")]
    Unauthorized,
    #[msg("Amount must be greater than 0")]
    AmountZero,
    #[msg("Market/AdjournMarket: Market is not finished")]
    MarketNotFinished,
    #[msg("Market/DraftMarket: Market key does exist")]
    MarketDoesExist,
    #[msg("Market/Bet: Market is not approved")]
    MarketNotApproved,
    #[msg("The maximum number of answers has been reached.")]
    MaxAnswersReached,
    #[msg("The answer key already exists.")]
    AnswerAlreadyExists,
    #[msg("The answer key does not exist.")]
    AnswerNotExists,
    #[msg("Market/SuccessMarket: Market does not contain answerKey")]
    MarketDoesNotContainAnswerKey,
    #[msg("Market/Receive: Cannot receive token")]
    CannotClaimToken,
    #[msg("Market/Receive: Answer key is not succeeded answer key")]
    AnswerKeyNotRight,
    #[msg("Invalid token bridge config")]
    InvalidTokenBridgeConfig,
    #[msg("Invalid token bridge authority signer")]
    InvalidTokenBridgeAuthoritySigner,
    #[msg("Invalid token bridge sequence")]
    InvalidTokenBridgeSequence,
    #[msg("Invalid token bridge wormhole bridge")]
    InvalidWormholeBridge,
    #[msg("Invalid token bridge wormhole fee collector")]
    InvalidWormholeFeeCollector,
    #[msg("Invalid token bridge wrapped mint")]
    InvalidTokenBridgeWrappedMint,
    #[msg("Invalid message")]
    InvalidMessage,
    #[msg("Invalid foreign emitter")]
    InvalidForeignEmitter,
    #[msg("Invalid token bridge emitter")]
    InvalidTokenBridgeEmitter,
    #[msg("Zero bridge amount")]
    ZeroBridgeAmount,
    #[msg("Invalid recipient")]
    InvalidRecipient,
    #[msg("Invalid relayer fee")]
    InvalidRelayerFee,
    #[msg("Invalid token bridge custody signer")]
    InvalidTokenBridgeCustodySigner,
    #[msg("Invalid foreign contract")]
    InvalidForeignContract
}

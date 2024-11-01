# Overview

Polyquest, Solana’s first SPL-based custom token-integrated prediction platform, is a cross-chain "predict to earn" DApp designed for communities, futures traders, and crypto influencers. In the 'SIGMA Sprint / The Global Multichain Builder Competition,' our focus is to expand our customer base to EVM users using Wormhole's cross-chain capabilities. Technically, we leverage Wormhole's CoreContract to allow users to make predictions on the Polyquest platform via a token-locking mechanism and the publishMessage method, which includes metadata related to users, tokens, and markets. To handle reward distribution for EVM users, we use Wormhole's TokenBridge to seamlessly transfer tokens from Solana to EVM when users make accurate predictions.

# Technical requirements

### Cross chain requirements:

- Utilize Wormhole for cross-chain betting based on its messaging service and enable cross-chain reward claims.

### User experience:

- Case \#1: Users on different chains should experience a seamless UX with no disruption.
- Case \#2: For tokens not attested on some destination chains, an efficient archiving solution based on Wormhole’s tech stack is required.

# How to archive the requirements

- Use Wormhole’s CoreContract to publish messages from other chains to the Solana contract.
- Use the Wormhole SDK to verify posted VAA data in Solana.
- Use Wormhole’s TokenBridge to facilitate smooth reward token transfers from Solana to other chains.

# Architect

### Main contract:

- WormholeGateway contract on the EVM chain
- polyquest_program in solana ( [Github](https://github.com/polyquest-solana/polyquest-solana-program) )

### System flow
![Bet](./img/cross_chain_system_flow.svg)

![Claim](./img/claim.svg)

### The user submit answer of a prediction in frontend

1. The user submits a prediction answer through the frontend.
2. The frontend calls the sendMessage function in the WormholeGateway contract on the EVM chain. This contract encodes the message and publishes it to Wormhole.
3. Wormhole returns the sequence of the VAA. The backend then uses this sequence to retrieve the VAA via an API call.
4. After obtaining the VAA hash, the backend requests Wormhole to create a postVaaAccount on Solana.
5. The backend calls the bet_cross_chain instruction in the Solana program, submitting the user’s answer to a market.
6. When the market closes, the reward value is calculated and marked as pending for the user to claim.
7. The user clicks the ‘Claim’ button, initiating a cross-chain transfer of their reward.

# Roadmap

- [x] 23/10: Develop a smart contract program on Base to lock a user's tokens and send a message containing metadata (user, quest, and token) via Wormhole message.
- [x] 25/10: Add a bet_cross_chain instruction to the Solana program, which utilizes metadata in the VaaAccount payload (created by Wormhole from a Base Contract message) to create a BetCrossChainAccount for a user on Base chain.
- [x] 27/10: Merge the existing code and add a method in Poly's SDK to enable placing bets for users on Base Sepolia.
- [x] 28/10: Test the betting feature for users on Base Sepolia.
- [x] 28/10: Add an instruction in the Solana program to reward the user on Base when they win, using Wormhole’s token bridge.
- [x] 30/10: Implement a function in the Base contract to process rewards for users, based on a VAA created via Wormhole's token bridge from Solana.
- [ ] 31/10: Merge the code and add a method in Poly’s SDK allowing users on Base to claim tokens when they win.
- [ ] 1/11: Test the token claim feature on Base to ensure smooth functionality.
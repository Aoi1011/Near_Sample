# Lantern

## Epoch

Each each epoch is 432,000 slots, if the slot number is divisible by 432,000, the it's at the boundary.

https://solana.stackexchange.com/questions/10382/whats-the-best-way-to-listen-for-epoch-updates

## Leader

`getSlotLeaders` keeps only 10 epochs worth of [schedules](https://github.com/anza-xyz/agave/blob/3863bb1bdf0f7c9a0b35c2c19dc50943ca39657e/ledger/src/leader_schedule_cache.rs#L22).
So instead of this endpoint, we will use `getBlock` endpoint with `rewards: true`

Optimistic light client
For each committee, take its members, concatencate them all together, and hash them into on hash. 
The prover(full node) then sends this sequence of hashes(one for each committee) to the client. 

Superlight clients
PoPos protocol requires each prover to organize its claimed sequence of committees - one per epoch - into a Merkle tree.

Solana
we can get the pubkey who votes by [`getVotesAccounts`](https://solana.com/docs/rpc/http/getvoteaccounts)

```
curl "https://mainnet.helius-rpc.com/?api-key=ff28efe6-4fe6-4cf5-9525-01adeed6ee0b" -X POST -H "Content-Type: application/json" -d '
                                {
                                  "jsonrpc": "2.0",
                                  "id": 2,
                                  "method": "getVoteAccounts",
                                  "params": [

                                  ]
                                }
                              ' | jq
```

## Epochs and Scheduling
Solana's leader schedule and epoch structure would need to be considered. Solana epochs have a fixed duration, and leaders (validators) are scheduled to write to the ledger in advance. An adaptation of the concept might involve creating Merkle trees for Solana's epochs, but the specific implementation would need to consider Solana's rapid block production and confirmation times.

## Validator Selection and Stake
Solana's consensus mechanism already incorporates stake-weighted voting, but the specifics of how validators are chosen and how stake influences consensus might affect how a bisection game or similar mechanism could be implemented.

## Proof of History Integration
Any implementation would need to consider how Proof of History, Solana's unique timekeeping mechanism, interacts with these proofs. PoH provides a way to verify the passage of time between two events, which could potentially be leveraged or need to be accounted for in a sublinear complexity proof system.

## Network and Performance Characteristics
Solana is designed for high throughput and low latency, with different network and performance characteristics than Ethereum. Any implementation would need to ensure that the additional complexity of sublinear proofs doesn't negatively impact these performance metrics.

Prover/Verifier model
The full nodes are trying to convince the light client of the system's state. In this context, the light client is known as the verifier and the full nodes  are known as the provers. 


Assumptions


The PoPos Primitive


## The Optimistic Light Client


## The Superlight Client


Merkle mountain ranges



## What is RPC?
RPC stands for Retemo Procedure Call. These are nodes that participate in the blockchain network and expose methods.

## How to work with RPCs?
To get data from the blockchain, you must make a JSON-RPC request to an RPC node participating in the network. 

## Document
- [What is ecdsa](https://zoom-blc.com/what-is-ecdsa)

## Resources
- https://www.nervos.org/knowledge-base/ultimate_guide_to_light_clients

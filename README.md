# Lantern

Papaer
- [Proofs of Proof-of-Stake with Sublinear Complexity](https://arxiv.org/pdf/2209.08673.pdf)

Optimistic light client
For each committee, take its members, concatencate them all together, and hash them into on hash. 
The prover(full node) then sends this sequence of hashes(one for each committee) to the client. 

Superlight clients
PoPos protocol requires each prover to organize its claimed sequence of committees - one per epoch - into a Merkle tree.

Solana
we can get the pubkey who votes by [`getVotesAccounts`](https://solana.com/docs/rpc/http/getvoteaccounts)

## What is RPC?
RPC stands for Retemo Procedure Call. These are nodes that participate in the blockchain network and expose methods.

## How to work with RPCs?
To get data from the blockchain, you must make a JSON-RPC request to an RPC node participating in the network. 

## Document
- [What is ecdsa](https://zoom-blc.com/what-is-ecdsa)

## Resources
- https://www.nervos.org/knowledge-base/ultimate_guide_to_light_clients

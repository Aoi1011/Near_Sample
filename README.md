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

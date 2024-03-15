# Proof of Proof of Stake

## Abstract
- Our proofs of proof-of-stake(PoPos) take the form of a Merkle tree of PoS epochs.
- The verifier enrolls the provers in a bisection game, in which honest provers are destined to win once an adversarial Merkle tree is challenged at sufficient depth.
- Promises both efficiency and decentralization by achieving logarithmic bootstrapping complexity. This is achieved through a novel construction using a Merkle tree of PoS epochs and engaging provers in a bisection game, where honest provers can prevail against adversarial attempts at deception when the Merkle tree is sufficiently challenged.

## 1. Introduction
- Efficiently verifying the proof-of-stake blockchain without downloading the whole PoS. -> proofs of proof of stake

### 1.1 Construction Overview

#### Full nodes
- A *full node* client boots holding only the genesis block and connects to other full nodes (known as provers) in order to synchronize to the latest tip. 
- The full node downloads the entire chain block-by-block verifying each transaction in the process.
- Once the client verifies the latest tip, it has calculated the latest state and can answer user queries such as "What is my balance?". 

#### Sync committees
- PoS protocols typically proceed in epochs during which the validator set is fixed.
- In each epoch, a subset of validators is elected by the protocol as the epoch committee.
- The current committee signs the latest state. Therefore, the client does not need to download all the blocks, but instead only needs to download the latest state and verify the committee signatures on it.
- To perform the verification at some later epoch, the client needs to keep track of the current committee.

#### Optimistic light client construction
- For each committee, take its members concatenate them all togerther, and hash them into one hash.
- The prover then sends this sequence of hashes (one for each committee) to the client. 
- If multiple provers five conflicting claims to the client, all  it needs to do is to find which one is truthful.
- If two provers disagree, the client focuses on the first point of disagreement in their hash sequences, and asks each prover to provide the respective handover signatures to substantiate their claims.

#### Superlight clients
- To achieve sublinear complexity, we improve the procedure to find the first point of disagreement.
- To this end, our final PoPos protocol rquires each prover to organize its claimed sequence of committees - one per epoch - into a merkle tree.


## 2. Preliminaries

**Proof-of-stake**
It is assumed that the majority of stake is honestly controlled at every point in time

**Primitives**

**Types of nodes**

validators 
- The stakeholders who participate in maintaining the system's consensus

full nodes
- do not participate in maintaining consensus, can join the system, download its full history, and discover its current state.

light clients
- insterested in joining the system and learning a small part of the sytem state (such as their user's balance) without downloading everything

**Time**
The protocol execution proceeds in discrete epochs.
Epochs are further subdivided into rounds, which correspond to shorter time durations during which a message sent by one honest party is  received by all others

**The prover/verifier model**
The bootstrapping process begins with a light client connecting to its full node peers to begin synchrnizing
the light client is known as the *verifier*
the full nodes are known as the *provers*

## 3. The PoPoS Primitive


## 4. The Optimistic Light Client

**Sync committees**



## Proof of Stake (PoS) superlight client

1. Merkle Tree of PoS Epochs
In a PoS blockchain, time is divided into epochs, which are periods during which a set of validators (provers) are responsible for validating transactions and creating new blocks. A Merkle tree is a cryptographic structure that efficiently summarizes all the data in a set, allowing for quick verification of whether a piece of data is part of the set. In this context, each node in the Merkle tree represents an epoch's state or summary. This allows the light client to quickly check the state of the blockchain at various points in time without needing to process every single block.

2. Bisection Game
This is a strategy to efficiently verify information. If a light client wants to verify the state of the blockchain at a specific epoch, it can challenge a prover (a full node) to provide proof. If the prover is dishonest, the light client can engage in a "bisection" process, where it asks the prover to provide proof for smaller and smaller intervals until it either finds a discrepancy or narrows down the interval to a point where it can directly verify the truth. Honest provers can quickly provide valid proofs, while dishonest ones are caught in their deception.


## Note

**difference between a full node and a validator**

Validators are nodes that are staked with SOL tokens and are responsible for validating transactions and blocks, voting on every block to ensure network security. Full nodes include nodes that are not staked and are responsible for serving the network to clients. As every node, staked or otherwise, maintains a full copy of the ledger, the security of the network is not dependent on the number of validators alone: any full node can be used to recover the state in case of a catastrophic failure.

**How a new validator get last state in Solana?**
https://solana.stackexchange.com/questions/2396/how-a-new-validator-get-last-state-in-solana

Papaer
- [Proofs of Proof-of-Stake with Sublinear Complexity](https://arxiv.org/pdf/2209.08673.pdf)

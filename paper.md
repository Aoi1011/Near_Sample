# Proof of Proof of Stake

## Proof of Stake (PoS) superlight client
Promises both efficiency and decentralization by achieving logarithmic bootstrapping complexity. This is achieved through a novel construction using a Merkle tree of PoS epochs and engaging provers in a bisection game, where honest provers can prevail against adversarial attempts at deception when the Merkle tree is sufficiently challenged.

1. Merkle Tree of PoS Epochs
In a PoS blockchain, time is divided into epochs, which are periods during which a set of validators (provers) are responsible for validating transactions and creating new blocks. A Merkle tree is a cryptographic structure that efficiently summarizes all the data in a set, allowing for quick verification of whether a piece of data is part of the set. In this context, each node in the Merkle tree represents an epoch's state or summary. This allows the light client to quickly check the state of the blockchain at various points in time without needing to process every single block.

2. Bisection Game
This is a strategy to efficiently verify information. If a light client wants to verify the state of the blockchain at a specific epoch, it can challenge a prover (a full node) to provide proof. If the prover is dishonest, the light client can engage in a "bisection" process, where it asks the prover to provide proof for smaller and smaller intervals until it either finds a discrepancy or narrows down the interval to a point where it can directly verify the truth. Honest provers can quickly provide valid proofs, while dishonest ones are caught in their deception.


